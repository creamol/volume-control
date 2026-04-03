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

        for _ in 0..6 {
            let enumerator: IMMDeviceEnumerator =
                match CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL) {
                    Ok(e) => e,
                    Err(_) => {
                        std::thread::sleep(std::time::Duration::from_secs(5));
                        continue;
                    }
                };

            if let Ok(device) = enumerator.GetDefaultAudioEndpoint(eRender, eConsole) {
                if let Ok(volume) = device.Activate::<IAudioEndpointVolume>(CLSCTX_ALL, None) {
                    if let Ok(current_volume) = volume.GetMasterVolumeLevelScalar() {
                        if (current_volume - 1.0_f32).abs() < 0.001 {
                            let _ = volume.SetMasterVolumeLevelScalar(0.16, &GUID::zeroed());
                            if let Ok(muted) = volume.GetMute() {
                                if bool::from(muted) {
                                    let _ = volume.SetMute(false, &GUID::zeroed());
                                }
                            }
                        }
                    }
                }
            }

            std::thread::sleep(std::time::Duration::from_secs(5));
        }
    }
}
