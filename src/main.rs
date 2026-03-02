// Cargo.toml 依赖:
// [dependencies]
// windows = { version = "0.58", features = [
//     "Win32_Media_Audio",
//     "Win32_Media_Audio_Endpoints",
//     "Win32_System_Com",
//     "Win32_UI_Shell_PropertiesSystem",
// ] }

use windows::{
    core::GUID,
    Win32::{
        Media::Audio::{
            eConsole, eRender, IAudioEndpointVolume, IMMDeviceEnumerator, MMDeviceEnumerator,
        },
        System::Com::{CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED},
    },
};

fn main() {
    unsafe {
        // 初始化 COM
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

        // 获取设备枚举器
        let enumerator: IMMDeviceEnumerator =
            match CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL) {
                Ok(e) => e,
                Err(e) => {
                    eprintln!("Error creating device enumerator: {e}");
                    return;
                }
            };

        // 获取默认音频输出设备（扬声器）
        let device = match enumerator.GetDefaultAudioEndpoint(eRender, eConsole) {
            Ok(d) => d,
            Err(e) => {
                eprintln!("Error getting speakers: {e}");
                return;
            }
        };

        // 获取音量控制接口
        let volume: IAudioEndpointVolume = match device.Activate(CLSCTX_ALL, None) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error activating volume interface: {e}");
                return;
            }
        };

        // 获取当前音量 (0.0 ~ 1.0)
        let current_volume = match volume.GetMasterVolumeLevelScalar() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Error getting volume: {e}");
                return;
            }
        };

        // 判断是否接近 100%
        if (current_volume - 1.0_f32).abs() < 0.001 {
            // 设为 16%
            if let Err(e) = volume.SetMasterVolumeLevelScalar(0.16, &GUID::zeroed()) {
                eprintln!("Error setting volume: {e}");
                return;
            }

            // 如果当前静音则取消静音
            if let Ok(muted) = volume.GetMute() {
                if muted.as_bool() {
                    let _ = volume.SetMute(false, &GUID::zeroed());
                }
            }
        }
    }
}