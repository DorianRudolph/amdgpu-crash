use std::ffi::CString;

pub use ash::version::{DeviceV1_0, EntryV1_0, InstanceV1_0};
use ash::vk;
use ash::prelude::VkResult;

use super::debug_reporter;
use super::VkDebugReporter;
use super::window_support;

/// Create one of these at startup. It never gets lost/destroyed.
pub struct VkInstance {
    pub entry: ash::Entry,
    pub instance: ash::Instance,
    pub debug_reporter: Option<VkDebugReporter>,
}

impl VkInstance {
    /// Creates a vulkan instance.
    pub fn new(app_name: &CString, use_vulkan_debug_layer: bool) -> VkResult<VkInstance> {

        // This loads the dll/so if needed
        info!("Finding vulkan entry point");
        //TODO: Return this error
        let entry = ash::Entry::new().expect("Could not find Vulkan entry point");

        // Determine the supported version of vulkan that's available
        let vulkan_version = match entry.try_enumerate_instance_version()? {
            // Vulkan 1.1+
            Some(version) => {
                let major = ash::vk_version_major!(version);
                let minor = ash::vk_version_minor!(version);
                let patch = ash::vk_version_patch!(version);

                (major, minor, patch)
            },
            // Vulkan 1.0
            None => (1, 0, 0),
        };

        info!("Found Vulkan version: {:?}", vulkan_version);

        // Get the available layers/extensions
        let layers = entry.enumerate_instance_layer_properties()?;
        debug!("Available Layers: {:#?}", layers);
        let extensions = entry.enumerate_instance_extension_properties()?;
        debug!("Available Extensions: {:#?}", extensions);

        // Expected to be 1.1.0 or 1.0.0 depeneding on what we found in try_enumerate_instance_version
        // https://vulkan.lunarg.com/doc/view/1.1.70.1/windows/tutorial/html/16-vulkan_1_1_changes.html
        let api_version = ash::vk_make_version!(vulkan_version.0, vulkan_version.1, 0);
        
        // Info that's exposed to the driver. In a real shipped product, this data might be used by
        // the driver to make specific adjustments to improve performance
        //TODO: Review docs on usage of api_version:
        // https://www.khronos.org/registry/vulkan/specs/1.1-extensions/man/html/VkApplicationInfo.html
        let appinfo = vk::ApplicationInfo::builder()
            .application_name(app_name)
            .application_version(0)
            .engine_name(app_name)
            .engine_version(0)
            .api_version(api_version);

        // Determine what layers to use
        let validation_layer_name = CString::new("VK_LAYER_LUNARG_standard_validation").unwrap();

        let mut layer_names = vec![];
        if use_vulkan_debug_layer {
            //TODO: Validate that the layer exists
            //if layers.iter().find(|x| CStr::from_bytes_with_nul(&x.layer_name) == &validation_layer_name) {
            layer_names.push(validation_layer_name);
            //}
        }

        let layers_names_raw: Vec<*const i8> = layer_names
            .iter()
            .map(|raw_name| raw_name.as_ptr())
            .collect();

        // Determine what extensions to use
        let extension_names_raw = window_support::extension_names();

        // Create the instance
        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&appinfo)
            .enabled_layer_names(&layers_names_raw)
            .enabled_extension_names(&extension_names_raw);

        info!("Creating vulkan instance");
        let instance: ash::Instance = unsafe {
            //TODO: Return this error
            entry
                .create_instance(&create_info, None)
                .expect("Instance creation error")
        };

        // Setup the debug callback for the validation layer
        let debug_reporter = if use_vulkan_debug_layer {
            Some(Self::setup_vulkan_debug_callback(&entry, &instance)?)
        } else {
            None
        };

        Ok(VkInstance {
            entry,
            instance,
            debug_reporter,
        })
    }

    /// This is used to setup a debug callback for logging validation errors
    fn setup_vulkan_debug_callback(entry: &ash::Entry, instance: &ash::Instance) -> VkResult<VkDebugReporter> {
        info!("Seting up vulkan debug callback");
        let debug_info = vk::DebugReportCallbackCreateInfoEXT::builder()
            .flags( //TODO: Allow configuring this
                vk::DebugReportFlagsEXT::ERROR
                    | vk::DebugReportFlagsEXT::WARNING
                    | vk::DebugReportFlagsEXT::PERFORMANCE_WARNING
                    | vk::DebugReportFlagsEXT::INFORMATION
                    | vk::DebugReportFlagsEXT::DEBUG,
            )
            .pfn_callback(Some(debug_reporter::vulkan_debug_callback));

        let debug_report_loader = ash::extensions::ext::DebugReport::new(entry, instance);
        let debug_callback = unsafe {
            debug_report_loader
                .create_debug_report_callback(&debug_info, None)?
        };

        Ok(VkDebugReporter {
            debug_report_loader,
            debug_callback
        })
    }
}

impl Drop for VkInstance {
    fn drop(&mut self) {
        debug!("destroying VkInstance");
        std::mem::drop(self.debug_reporter.take());

        unsafe {
            self.instance.destroy_instance(None);
        }

        debug!("destroyed VkInstance");
    }
}

