# ronskynet.xyz
### This repo contains the /api/v2 code, in rust

## API Description

### Endpoints

Note: The endpoints do not return data 100% the same as the v1 api, always check data before modification.

/posts
: Get all posts

/posts/sorted
: Get posts with defaults, querystring args are sortBy, direction and limit
: `curl http://<host>/posts/sorted?sortBy=id&direction=desc&limit=10`

/posts/{id}
: Get full post by it's id

/users/{user_id}/posts
: Get posts by user id, where user id is the post's creator

/users/{user_id/posts/sorted
: Same principle as /posts/sorted and same querystring args
: `curl http://<host>/users/{user_id}/posts/sorted?sortBy=id&direction=desc&limit=10`
