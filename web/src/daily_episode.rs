use std::rc::Rc;

use gloo_net::http::Request;

pub async fn get_day_episode() -> Result<usize, Rc<gloo_net::Error>> {
    let url = (env!("API_URL")).to_string() + "/episode";

    let req_res = Request::get(url.as_str()).send().await;

    match req_res {
        Ok(res) => {
            let json = res.json::<usize>().await;
            match json {
                Ok(ep_idx) => Ok(ep_idx),
                Err(e) => Err(Rc::new(e)),
            }
        }
        Err(e) => Err(Rc::new(e)),
    }
}
