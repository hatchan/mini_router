use std::fmt::Display;

fn main() {
    // let user_url = "/settings/profile";
    // let user_url = "/settings/password";
    let user_url = "/orders/123";
    let result = RootRouter::parse_url(&user_url);

    match result {
        Some(route) => eprintln!("User navigated to: {route};"),
        None => todo!(),
    };
}

#[derive(Debug, PartialEq, Eq)]
pub enum RootRouter {
    // Optional enum
    // URL: /settings/{settings_page}
    Settings(Option<SettingsPage>),

    // Subrouter
    // URL: /orders/*
    Orders(OrderRouter),

    // Struct
    // URL: /users/{user_id}{?details=bool}
    User(UserRoute),
}

impl RootRouter {
    pub fn parse_url(url: &str) -> Option<Self> {
        if url.starts_with("/settings") {
            let remainder = url.len() - 9;
            let settings_page = if remainder > 7 && &url[10..17] == "profile" {
                Some(SettingsPage::Profile)
            } else if remainder > 8 && &url[10..18] == "password" {
                Some(SettingsPage::Password)
            } else {
                None
            };

            return Some(RootRouter::Settings(settings_page));
        } else if url.starts_with("/orders") {
            let sub_url = &url[7..];
            let sub_router = OrderRouter::parse_url(sub_url).expect("TODO: Invalid URL");

            return Some(RootRouter::Orders(sub_router));
        } else if url.starts_with("/users") {
            let sub_url = &url[7..];
            let user_id: u64 = sub_url.parse().expect("invalid user id");
            let user_route = UserRoute { user_id };
            return Some(RootRouter::User(user_route));
        } else {
            None
        }
    }

    pub fn to_url(&self) -> String {
        todo!()
    }
}

impl Display for RootRouter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RootRouter::Settings(settings_page) => {
                f.write_str("settings page")?;
                if let Some(settings_page) = settings_page {
                    f.write_fmt(format_args!(" - {}", settings_page))?;
                };
                Ok(())
            }
            RootRouter::Orders(order_router) => {
                f.write_str("orders page")?;
                f.write_fmt(format_args!(" - {}", order_router))
            }
            RootRouter::User(user_route) => {
                f.write_str("user page")?;
                f.write_fmt(format_args!(" - {}", user_route.user_id))
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SettingsPage {
    Profile,
    Password,
}

impl Display for SettingsPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingsPage::Profile => f.write_str("profile"),
            SettingsPage::Password => f.write_str("password"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum OrderRouter {
    // Empty enum
    // URL: /
    Overview,

    // Simple type
    // URL: /{order_id}
    Details(u64),
}

impl OrderRouter {
    pub fn parse_url(url: &str) -> Option<Self> {
        if url == "/" {
            Some(OrderRouter::Overview)
        } else {
            let remainder = &url[1..];
            let parsed: Result<u64, _> = remainder.parse();

            match parsed {
                Ok(num) => Some(OrderRouter::Details(num)),
                Err(_) => None,
            }
        }
    }

    pub fn to_url(&self) -> String {
        todo!()
    }
}

impl Display for OrderRouter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrderRouter::Overview => f.write_str("overview page"),
            OrderRouter::Details(user_id) => {
                f.write_fmt(format_args!("details page (order id: {user_id})"))
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct UserRoute {
    pub user_id: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_test() {
        let tests = vec![
            ("/settings", RootRouter::Settings(None)),
            ("/settings/", RootRouter::Settings(None)),
            (
                "/settings/profile",
                RootRouter::Settings(Some(SettingsPage::Profile)),
            ),
            (
                "/settings/password",
                RootRouter::Settings(Some(SettingsPage::Password)),
            ),
            ("/orders/", RootRouter::Orders(OrderRouter::Overview)),
            ("/orders/123", RootRouter::Orders(OrderRouter::Details(123))),
            ("/users/123", RootRouter::User(UserRoute { user_id: 123 })),
        ];

        for (url, expected_route) in tests {
            eprintln!("Parsing URL: {url}");
            let actual_route = RootRouter::parse_url(url).expect("should be valid");

            assert_eq!(actual_route, expected_route);

            eprintln!("=============================")
        }
    }
}
