use log::{info, warn};

#[cfg(target_os = "linux")]
pub fn authenticate_user(reason: &str) -> bool {
    info!("Requesting Linux PAM authentication for: {}", reason);
    let service_name = "passclip";
    
    let username = match std::env::var("USER") {
        Ok(u) => u,
        Err(_) => return false,
    };

    let mut authenticator = match pam_client::Context::new(service_name, Some(&username), pam_client::Flag::empty()) {
        Ok(ctx) => ctx,
        Err(e) => {
            warn!("Failed to initialize PAM context: {:?}", e);
            return false;
        }
    };

    authenticator.authenticate(pam_client::Flag::empty()).is_ok()
}

#[cfg(target_os = "macos")]
pub fn authenticate_user(reason: &str) -> bool {
    use objc2_foundation::NSString;
    use objc2_local_authentication::{LAContext, LAPolicy};

    info!("Requesting macOS LocalAuthentication (TouchID/Password) for: {}", reason);

    unsafe {
        let context = LAContext::new();
        let reason_ns = NSString::from_str(reason);
        let mut error: *mut objc2_foundation::NSError = std::ptr::null_mut();

        let can_evaluate = context.canEvaluatePolicy_error(
            LAPolicy::DeviceOwnerAuthentication,
            &mut error,
        );

        if !can_evaluate {
            warn!("LocalAuthentication policy check failed");
            return false;
        }

        let (tx, rx) = std::sync::mpsc::channel();
        let block = block2::RcBlock::new(move |success: bool, _err: *mut objc2_foundation::NSError| {
            let _ = tx.send(success);
        });

        context.evaluatePolicy_localizedReason_reply(
            LAPolicy::DeviceOwnerAuthentication,
            &reason_ns,
            &block,
        );

        rx.recv().unwrap_or(false)
    }
}

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
pub fn authenticate_user(reason: &str) -> bool {
    info!("Fallback authentication prompt: {}", reason);
    println!("[PassClip Authorization] {}", reason);
    println!("Type 'y' or 'yes' to allow access to secret:");

    let mut input = String::new();
    if std::io::stdin().read_line(&mut input).is_ok() {
        let trimmed = input.trim().to_lowercase();
        return trimmed == "y" || trimmed == "yes";
    }

    false
}