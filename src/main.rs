use windows::{
    core::GUID,
    Win32::{
        Media::Audio::{
            eConsole, eRender, IMMDeviceEnumerator, MMDeviceEnumerator,
        },
        Media::Audio::Endpoints::IAudioEndpointVolume,
        System::Com::{CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED},
    },
};

fn main() {
    unsafe {
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);

        let enumerator: IMMDeviceEnumerator =
            match CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL) {
                Ok(e) => e,
                Err(e) => { eprintln!("Error: {e}"); return; }
            };

        let device = match enumerator.GetDefaultAudioEndpoint(eRender, eConsole) {
            Ok(d) => d,
            Err(e) => { eprintln!("Error: {e}"); return; }
        };

        let volume: IAudioEndpointVolume = match device.Activate(CLSCTX_ALL, None) {
            Ok(v) => v,
            Err(e) => { eprintln!("Error: {e}"); return; }
        };

        let current_volume: f32 = match volume.GetMasterVolumeLevelScalar() {
            Ok(v) => v,
            Err(e) => { eprintln!("Error: {e}"); return; }
        };

        if (current_volume - 1.0_f32).abs() < 0.001 {
            if let Err(e) = volume.SetMasterVolumeLevelScalar(0.16, &GUID::zeroed()) {
                eprintln!("Error: {e}"); return;
            }
            if let Ok(muted) = volume.GetMute() {
                if bool::from(muted) {
                    let _ = volume.SetMute(false, &GUID::zeroed());
                }
            }
        }
    }
}
