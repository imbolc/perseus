use perseus::{
    http::header::{HeaderMap, HeaderName},
    GenericNode, RenderFnResultWithCause, Template,
};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use sycamore::prelude::{component, template, Template as SycamoreTemplate};

#[derive(Serialize, Deserialize, Debug)]
pub struct IndexPageProps {
    pub greeting: String,
}

#[component(IndexPage<G>)]
pub fn index_page(props: IndexPageProps) -> SycamoreTemplate<G> {
    template! {
        p {(props.greeting)}
        a(href = "about", id = "about-link") { "About!" }
    }
}

pub fn get_template<G: GenericNode>() -> Template<G> {
    Template::new("index")
        .build_state_fn(Rc::new(get_build_props))
        .template(Rc::new(|props: Option<String>| {
            template! {
                IndexPage(
                    serde_json::from_str::<IndexPageProps>(&props.unwrap()).unwrap()
                )
            }
        }))
        .head(Rc::new(|_| {
            template! {
                title { "Index Page | Perseus Example – Basic" }
            }
        }))
        .set_headers_fn(Rc::new(set_headers))
}

pub async fn get_build_props(_path: String, _locale: String) -> RenderFnResultWithCause<String> {
    Ok(serde_json::to_string(&IndexPageProps {
        greeting: "Hello World!".to_string(),
    })?) // This `?` declares the default, that the server is the cause of the error
}

pub fn set_headers(_props: Option<String>) -> HeaderMap {
    let mut map = HeaderMap::new();
    map.insert(
        HeaderName::from_lowercase(b"x-test").unwrap(),
        "custom value".parse().unwrap(),
    );
    map
}
