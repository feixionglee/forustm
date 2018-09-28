use super::super::{Article, NewArticleStats, Permissions, RUser, UserNotify, WebContext};
use super::super::{Postgresql, Redis};
use super::super::models::CommentWithNickName;
use super::super::page_size;
use super::super::{get_real_ip_from_req, get_ruser_from_session, get_user_agent_from_req};
use sapper::{Request, Response, Result as SapperResult, SapperModule, SapperRouter};
use sapper_std::{render, PathParams};
use uuid::Uuid;

pub struct WebArticle;

impl WebArticle {
    fn article(req: &mut Request) -> SapperResult<Response> {
        let mut web = req.ext().get::<WebContext>().unwrap().clone();

        let params = get_path_params!(req);
        let id: Result<Uuid, _> = t_param!(params, "id").parse();
        if let Err(e) = id {
            return res_400!(format!("UUID invalid: {}", e));
        }
        let redis_pool = req.ext().get::<Redis>().unwrap();
        let pg_conn = req.ext().get::<Postgresql>().unwrap().get().unwrap();
        let id = id.unwrap();
        let res = Article::query_article(&pg_conn, id);
        match res {
            Ok(r) => {
                let session_user = get_ruser_from_session(req);
                // create article view record
                let article_stats = NewArticleStats {
                    article_id: r.id,
                    ruser_id: session_user.clone().map(|user| user.id),
                    user_agent: get_user_agent_from_req(req),
                    visitor_ip: get_real_ip_from_req(req),
                };
                article_stats.insert(&pg_conn).unwrap();

                // remove user's notify about this article
                if let Some(user) = session_user.clone() {
                    UserNotify::remove_notifys_for_article(user.id, r.id, &redis_pool);
                    let user_notifys = UserNotify::get_notifys(user.id, &redis_pool);
                    web.add("user_notifys", &user_notifys);
                }

                // article
                web.add("res", &r);

                // author
                let author = RUser::query_with_id(&pg_conn, r.author_id).unwrap();
                web.add("author", &author);

                // comments
                let page = 1;
                let comments = CommentWithNickName::comments_with_article_id_paging(
                    &pg_conn,
                    id,
                    page,
                    page_size(),
                );
                match comments {
                    Ok(com) => {
                        web.add("page_size", &page_size());
                        web.add("page", &page);

                        web.add("comments", &com.comments);
                        web.add("total", &com.total);
                        web.add("max_page", &com.max_page);

                        res_html!("detailArticle.html", web)
                    }
                    Err(e) => res_500!(e),
                }
            }
            Err(e) => res_400!(format!("article not found: {}", e)),
        }
    }

    fn edit(req: &mut Request) -> SapperResult<Response> {
        let web = req.ext().get::<WebContext>().unwrap().clone();
        match *req.ext().get::<Permissions>().unwrap() {
            Some(_) => res_html!("editArticle.html", web),
            None => res_redirect!("/login"),
        }
    }
}

impl SapperModule for WebArticle {
    fn router(&self, router: &mut SapperRouter) -> SapperResult<()> {
        router.get("/user/article/edit", WebArticle::edit);
        router.get("/article/:id", WebArticle::article);

        Ok(())
    }
}
