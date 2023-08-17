use autocxx::prelude::*;
use cxx::let_cxx_string;
use std::pin::Pin;

include_cpp! {
    #include "wrapper.h"
    safety!(unsafe)
    generate!("Model")
}

#[repr(transparent)]
pub struct Model(Pin<Box<ffi::Model>>);

impl Model {
    pub fn from_json(filename: &str) -> Self {
        let_cxx_string!(filename_cxx = "");
        filename_cxx.as_mut().push_str(filename);
        let inner_model = ffi::Model::new(&filename_cxx).within_box();
        Self(inner_model)
    }

    pub fn forward(&mut self, input: &[f32]) -> &[f32] {
        let input_ptr = input.as_ptr();
        let _ = unsafe { self.0.as_mut().forward(input_ptr) };
        let output = self.0.as_mut().getOutputs();
        let output_size = self.0.as_mut().getOutSize().0 as usize;
        let output = unsafe { std::slice::from_raw_parts(output, output_size) };
        output
    }
}

// TODO remove this and relly on auto impl
unsafe impl Send for Model {}
unsafe impl Sync for Model {}

#[cfg(test)]
mod test {
    #[test]
    fn test_create_model_float() {
        let mut model = super::Model::from_json("vendor/RTNeural/models/full_model.json");
        for i in 0..100 {
            let i = i as f32 * 1.0;
            let output = model.forward(&[i, i, i]);
            assert_eq!(output.len(), 1);
        }
    }

    #[test]
    fn test_send_sync() {
        fn is_send_sync<T: Send + Sync>() {}
        is_send_sync::<super::Model>();
    }
}
