use ash;
use ash::vk;
use std::ffi::CString;

use crate::core::platforms;

pub struct Context {
    pub instance : ash::Instance,
}

impl Context {

    unsafe extern "system" fn debug_log_vulkan(_msg_sev: vk::DebugUtilsMessageSeverityFlagsEXT, _msg_type: vk::DebugUtilsMessageTypeFlagsEXT, _callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT, _log_msg: *mut std::ffi::c_void) -> u32 {
        0
    }

    fn create_instance(entry: &ash::Entry, app_name: &str, app_version: u32) -> ash::Instance {
        let app_name = CString::new(app_name).unwrap();
        let eng_name = CString::new("aster").unwrap();
        
        let app_info = vk::ApplicationInfo::builder()
            .api_version(vk::API_VERSION_1_2)
            .application_name(app_name.as_c_str())
            .application_version(app_version)
            .engine_name(eng_name.as_c_str())
            .engine_version(app_version);

        let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
            .message_severity(vk::DebugUtilsMessageSeverityFlagsEXT::ERROR | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING | vk::DebugUtilsMessageSeverityFlagsEXT::INFO)
            .message_type(vk::DebugUtilsMessageTypeFlagsEXT::GENERAL | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION)
            .pfn_user_callback(Some(Context::debug_log_vulkan));

        let exts = platforms::required_extension_names();
            
        let create_info = vk::InstanceCreateInfo::builder()
            .application_info(&app_info)
            .enabled_extension_names(&*exts);

        let instance : ash::Instance = unsafe {
            entry
                .create_instance(&create_info, None)
                .expect("Failed to create instance!")
        };

        instance
    }

    pub fn new(entry: &ash::Entry, app_name: &str, app_version: u32) -> Context {
        Context {
            instance: Context::create_instance(&entry, &app_name, app_version)
        }
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { self.instance.destroy_instance(None); }
    }
}