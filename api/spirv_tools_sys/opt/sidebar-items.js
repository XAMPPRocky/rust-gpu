initSidebarItems({"enum":[["Passes",""]],"fn":[["optimizer_create",""],["optimizer_destroy",""],["optimizer_options_create","Creates an optimizer options object with default options. Returns a valid options object. The object remains valid until it is passed into |spvOptimizerOptionsDestroy|."],["optimizer_options_destroy","Destroys the given optimizer options object."],["optimizer_options_preserve_bindings","Records whether all bindings within the module should be preserved."],["optimizer_options_preserve_spec_constants","Records whether all specialization constants within the module should be preserved."],["optimizer_options_run_validator","Records whether or not the optimizer should run the validator before optimizing.  If |val| is true, the validator will be run."],["optimizer_options_set_max_id_bound","Records the maximum possible value for the id bound."],["optimizer_options_set_validator_options","Records the validator options that should be passed to the validator if it is run."],["optimizer_register_hlsl_legalization_passes","Registers passes that attempt to legalize the generated code."],["optimizer_register_pass",""],["optimizer_register_performance_passes","Registers passes that attempt to improve performance of generated code. This sequence of passes is subject to constant review and will change from time to time."],["optimizer_register_size_passes","Registers passes that attempt to improve the size of generated code. This sequence of passes is subject to constant review and will change from time to time."],["optimizer_register_vulkan_to_webgpu_passes","Registers passes that have been prescribed for converting from Vulkan to WebGPU. This sequence of passes is subject to constant review and will change from time to time."],["optimizer_register_webgpu_to_vulkan_passes","Registers passes that have been prescribed for converting from WebGPU to Vulkan. This sequence of passes is subject to constant review and will change from time to time."],["optimizer_run",""]],"struct":[["Optimizer",""],["OptimizerOptions",""]]});