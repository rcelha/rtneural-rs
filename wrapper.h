#pragma once

#include "vendor/RTNeural/RTNeural/RTNeural.h"
#include <memory>

typedef RTNeural::Model<float> ModelF32;

class Model {
public:
  Model(const std::string &fileName) {
    std::ifstream jsonStream(fileName, std::ifstream::binary);
    model = RTNeural::json_parser::parseJson<float>(jsonStream);
    model->reset();
  }

  inline float forward(const float *input) { return model->forward(input); }

  inline const float *getOutputs() { return model->getOutputs(); }

  inline int getOutSize() { return model->getOutSize(); }

private:
  std::unique_ptr<ModelF32> model;
};
