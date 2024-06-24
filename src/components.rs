//! UI Components. [Component] trait is object-safe, allowing very nice
//! component composition patterns via Rust's dynamic dispatch features.

// In many cases, we need to do a let binding to satisfy the borrow checker
// and for some reason, clippy identifies those as unnecessary. Maybe there
// are and clippy knows more than me, maybe not.
#![allow(clippy::let_and_return)]

use super::{auth::is_anon, prelude::*};

#[cfg(feature = "live_reload")]
const LIVE_RELOAD_SCRIPT: &str = r#"<script>
    (async () => {
        while (true) {
            try {
                await fetch('/ping?poll_interval_secs=60');
            } catch (e) {
                console.log("hup from ping; let's live-reload");
                const el = document.createElement('p');
                el.innerText = "Reloading...";
                el.classList.add("bg-yellow-100");
                el.classList.add("p-2");
                el.classList.add("rounded");
                el.classList.add("w-full");
                el.classList.add("dark:text-black");
                document.body.insertBefore(el, document.body.firstChild);
                setInterval(async () => {
                    setTimeout(() => {
                        // At some point, a compiler error may be preventing
                        // the server from coming back
                        el.innerText = "Reload taking longer than usual; check for a compiler error";
                    }, 5000);
                    // Now the server is down, we'll fast-poll it (trying to
                    // get an immediate response), and reload the page when it
                    // comes back
                    try {
                        await fetch('/ping?poll_interval_secs=0');
                        window.location.reload()
                    } catch (e) {}
                }, 100);
                break;
            }
        }
    })();
</script>"#;

#[cfg(not(feature = "live_reload"))]
const LIVE_RELOAD_SCRIPT: &str = "";

pub trait Component: Send + Sync {
    /// Render the component to a HTML string. By convention, the
    /// implementation should sanitize all string properties at render-time
    fn render(&self) -> String;
}

impl std::fmt::Display for dyn Component {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}

pub struct Page<'a> {
    pub title: &'a str,
    pub children: &'a dyn Component,
}

impl Component for Page<'_> {
    fn render(&self) -> String {
        // note: we'll get a compiler error here until the tailwind build
        // occurs. Make sure you use `make build` in the Makefile to get
        // both to happen together
        let tailwind = include_str!("./tailwind.generated.css");
        let htmx = Route::Htmx;
        let apple_icon = Route::StaticAppleIcon;
        let manifest = Route::StaticManifest;
        format!(
            r##"<!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0"></meta>
                    <meta name="theme-color" content="#BBF7D0"/>
                    <meta name="description" content="ChatGPT-powered calorie counter" />
                    <title>{title}</title>
                    <style>
                        {tailwind}
                    </style>
                    {LIVE_RELOAD_SCRIPT}
                    <link rel="manifest" href="{manifest}" />
                    <link rel="apple-touch-icon" href="{apple_icon}">
                    <script defer src="{htmx}"></script>
                </head>
                <body hx-boost="true">
                    {body_html}
                </body>
            </html>
            "##,
            tailwind = tailwind,
            title = clean(self.title),
            body_html = self.children.render()
        )
    }
}

struct Footer;
impl Component for Footer {
    fn render(&self) -> String {
        let privacy = Route::PrivacyPolicy;
        let tos = Route::TermsOfService;
        let home = Route::UserHome;
        let about = Route::About;
        format!(
            r#"
            <footer class="flex flex-wrap items-center justify-center gap-2 p-4">
                <a class="link" href="{privacy}">Privacy Policy</a>
                <a class="link" href="{tos}">Terms of Service</a>
                <a class="link" href="{home}">Dashboard</a>
                <a class="link" href="/">Home</a>
                <a class="link" href="{about}">About</a>
            </footer>
            "#
        )
    }
}

pub struct PageContainer<'a> {
    pub children: &'a dyn Component,
}
impl Component for PageContainer<'_> {
    fn render(&self) -> String {
        let children = self.children.render();
        let footer = Footer {}.render();
        format!(
            r#"
            <div
                class="p-2 sm:p-4 md:p-8 bg-teal-50 dark:bg-indigo-1000
                dark:text-slate-200 min-h-[100vh]"
            >
                {children}
                {footer}
            </div>
            "#
        )
    }
}

pub struct Home {}
impl Component for Home {
    fn render(&self) -> String {
        let login_route = Route::Login;
        let register_route = Route::Register;
        let footer = Footer {}.render();
        format!(
            r#"
            <main
                class="p-2 sm:p-4 md:p-8 bg-teal-50 dark:bg-indigo-1000
                dark:text-slate-200 min-h-[100vh]"
            >
                <h1 class="mt-2 md:mt-8 text-3xl font-extrabold">
                    &#127793; OG &#129752;
                </h1>
                <p>
                    Orton-Gillingham Lesson Planner
                </p>
                <a class="link" href="{login_route}">Login</a>
                <a class="link" href="{register_route}">Register</a>
            </main>
            {footer}
            "#
        )
    }
}
pub struct TrialAccountCounter {
    count_remaining: usize,
}
impl Component for TrialAccountCounter {
    fn render(&self) -> String {
        let count_remaining = self.count_remaining;
        format!(
            r#"
            <span hx-trigger="load delay:5s">{count_remaining}</span>
            "#
        )
    }
}

pub struct ExternalLink<'a> {
    pub href: &'a str,
    pub children: Box<dyn Component>,
}
impl Component for ExternalLink<'_> {
    fn render(&self) -> String {
        let children = self.children.render();
        let href = clean(self.href);
        format!(
            r#"
            <a href={href}>
                {children}
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-3 h-3 inline">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 19.5l15-15m0 0H8.25m11.25 0v11.25" />
                </svg>
            </a>
            "#
        )
    }
}

pub struct UserHome<'a> {
    pub username: &'a str,
}
impl Component for UserHome<'_> {
    fn render(&self) -> String {
        let username = clean(self.username);
        let log_out = Route::Logout;
        let register_route = Route::Register;
        let register_ui = if is_anon(self.username) {
            format!(
                r#"
            <p class="text-sm">
                You are an anonymous user. It was quick and easy to jump into
                the app, but you should register for an account to create a
                username and password. Your user ID won't change, so you'll
                retain ownership of any data you create while anonymously
                trying the app if you convert into a user.
            </p>
            <p class="text-sm">
                Note that if you allow an anonymous user to sign out, they will
                lose everything, so try to avoid that! I normally hide the
                "log out" button from anonymous users, and only show it after
                registration.
            </p>
            <a href="{register_route}">
                <button class="bg-emerald-100 hover:bg-emerald-200 rounded p-1">
                    Register
                </button>
            </a>
            "#
            )
        } else {
            "".into()
        };
        format!(
            r#"
            <div class="flex flex-col prose">
                Hi, {username}!
                <a class="link" href="{log_out}">Log Out</a>
                {register_ui}
            </div>
            "#
        )
    }
}

pub struct Saved<'a> {
    pub message: &'a str,
}
impl Component for Saved<'_> {
    fn render(&self) -> String {
        let void = Route::Void;
        let message = clean(self.message);
        format!(
            r##"
            <div
                hx-get="{void}"
                hx-trigger="load delay:2s"
                class="fixed top-2 my-2 bg-slate-200 dark:bg-emerald-800 p-2
                rounded-xl"
                >
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                    stroke-width="1.5"
                    stroke="currentColor"
                    class="inline bg-emerald-100 dark:bg-emerald-600 p-2
                    rounded-full w-8 h-8"
                >
                  <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12.75l6 6 9-13.5" />
                </svg>
                {message}
            </div>
            "##
        )
    }
}

pub struct AboutPage;
impl Component for AboutPage {
    fn render(&self) -> String {
        let home = Route::UserHome;
        format!(
            r#"
            <div class="prose dark:text-slate-200">
                <h1 class="dark:text-slate-200">About Your App</h1>
                <p><a class="link" href="{home}">Return Home</a></p>
                <p>
                    Tell the world about your app!
                </p>
            </div>
            "#
        )
    }
}

pub struct AnonWarning;
impl Component for AnonWarning {
    fn render(&self) -> String {
        let register = Route::Register;
        format!(
            r#"
            <div class="flex items-center justify-center">
                <a href="{register}">
                    <div
                        class="text-black text-xs inline-block bg-yellow-100
                        p-1 rounded-lg my-2 max-w-prose"
                    >
                        <h1 class="text-lg font-bold text-center">
                            Anon Warning
                        </h1>
                        <p class="text-base">
                            You're still registered as an anonymous user, which
                            means that you haven't shared a username, email, or
                            password. If you reset your cookies, move to a
                            different device, or loose your device, your account
                            cannot be recovered! Click here to register your
                            account so that you can create a password, login on
                            multiple devices, or use your email to recover your
                            account in case you forget your password.
                        </p>
                        <p class="text-lg">Click here to register!</p
                        >
                    </div>
                </a>
            </div>
            "#
        )
    }
}

pub struct Span {
    pub content: String,
}
impl Component for Span {
    fn render(&self) -> String {
        format!("<span>{}</span>", clean(&self.content))
    }
}
