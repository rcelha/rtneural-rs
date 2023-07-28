use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use rtneural_rs::Model;
use std::sync::Arc;
use utils::ensure_models_folder;

pub mod editor;
pub mod utils;

pub struct Tweed {
    pub params: Arc<TweedParams>,
    pub model: Option<Model>,
    pub prev_selected_model: i32,
    pub model_list: Vec<String>,
}

impl Default for Tweed {
    fn default() -> Self {
        let model_list =
            ensure_models_folder().expect("Couldn't not create the temp folder with models");
        let params: TweedParams = TweedParams::new(model_list.len() - 1);

        Self {
            model: None,
            params: Arc::new(params),
            prev_selected_model: -1,
            model_list,
        }
    }
}

impl Tweed {
    pub fn load_model(&mut self, index: usize) {
        let json_path = self.model_list.get(index).expect("Index out of range");
        let model = Model::from_json(&json_path);
        self.model.replace(model);
    }
}

impl Plugin for Tweed {
    const NAME: &'static str = "Tweed";
    const URL: &'static str = "https://github.com/rcelha/rtneural-rs";
    const EMAIL: &'static str = "dev@null.com";
    const VENDOR: &'static str = "Its a me";
    const VERSION: &'static str = "0.0.1";
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];
    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;
    // const HARD_REALTIME_ONLY: bool = true;
    const SAMPLE_ACCURATE_AUTOMATION: bool = false;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(self.params.clone(), self.params.editor_state.clone())
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let selected_model = self.params.model.value();
        if self.prev_selected_model != selected_model {
            self.load_model(selected_model as usize);
            self.prev_selected_model = selected_model;
        }

        for channel_samples in buffer.iter_samples() {
            let gain = self.params.gain.smoothed.next();

            for sample in channel_samples {
                *sample *= gain;
                match self.model.as_mut() {
                    Some(model) => {
                        *sample = model.forward(&mut [*sample])[0];
                    }
                    _ => (),
                }
            }
        }

        ProcessStatus::Normal
    }

    fn deactivate(&mut self) {}
}

impl Vst3Plugin for Tweed {
    const VST3_CLASS_ID: [u8; 16] = *b"Tweed00000000000";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

impl ClapPlugin for Tweed {
    const CLAP_ID: &'static str = "com.example.tweed";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A Tweed amp");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
    ];
}

#[derive(Params)]
pub struct TweedParams {
    #[id = "gain"]
    pub gain: FloatParam,

    #[id = "model"]
    pub model: IntParam,

    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,
}

impl TweedParams {
    pub fn new(num_models: usize) -> Self {
        let gain = FloatParam::new(
            "gain",
            1.0,
            FloatRange::Linear {
                min: 0.01,
                max: 2.0,
            },
        );
        let model = IntParam::new(
            "model",
            0,
            IntRange::Linear {
                min: 0,
                max: num_models as i32,
            },
        );
        let editor_state = editor::default_state();

        Self {
            gain,
            model,
            editor_state,
        }
    }
}

nih_export_clap!(Tweed);
nih_export_vst3!(Tweed);
