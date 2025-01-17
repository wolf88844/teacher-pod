
use dioxus::prelude::*;

use crate::TOAST;

pub fn Login()->Element{
    let toast = TOAST.signal();

    let email = use_signal(||String::new);
    let password = use_signal(||String::new);

    rsx!(
        div{
            class:"justify-center",
            div{
                class:"p-10 xs:p-0 mx-auto md:w-full md:max-w-md",
                h1{
                    class:"font-bold text-center text-black dark:text-white text-2x mb-5",
                    "Sign In | TeacherPod"
                }
            }
        }
    )
}