# api

## USERS SERVICE
| NAMESPACE | METHOD | ROUTE | FUNCTIONALITY | REQUIRE | RESPONSE SUCCESS | RESPONSE FAIL |
| --------- | -------| ----- | ------------- | ---------------- |---------------- | ------------- |
| USER | *GET* | ```v1/user/``` | _List All Users_ |Authorization Header, Role = 0| [<br>&nbsp;{ <br>&nbsp;&nbsp;user_id : int, <br>&nbsp;&nbsp;username: String, <br>&nbsp;&nbsp;firstname: String, <br>&nbsp;&nbsp;lastname: String, <br>&nbsp;&nbsp;email: String,<br>&nbsp;&nbsp;phone: String, <br>&nbsp;&nbsp;created_at: DateTime, <br>&nbsp;&nbsp;updated_at: DateTime, <br>&nbsp;&nbsp;status: int<br>&nbsp;}<br>] | FAIL |
|  | *POST* | ```v1/user/``` | _Create A User_ |Authorization Header, Role = 0| OK | FAIL |
|  | *GET* | ```v1/user/{id}``` | _Get User By ID_ |Authorization Header| { <br>&nbsp;user_id : int, <br>&nbsp;username: String, <br>&nbsp;firstname: String, <br>&nbsp;lastname: String, <br>&nbsp;email: String,<br>&nbsp;phone: String, <br>&nbsp;created_at: DateTime, <br>&nbsp;updated_at: DateTime, <br>&nbsp;status: int<br>} | FAIL |
|  | *PUT* | ```v1/user/{id}``` | _Update A User_ |Authorization Header| OK | FAIL |
|  | *DELETE* | ```v1/user/{id}``` | _Delete A User_ |Authorization Header| OK | FAIL | 
| DEPARTMENT | *POST* | ```v1/department/``` | _Create A Department_ |Authorization Header| OK | FAIL |
|  | *GET* | ```v1/department/{id}``` | _Get Deparment By ID_ |Authorization Header| {<br>&nbsp;department_id : int, <br>&nbsp;department_name: String, <br>&nbsp;created_by: int <br>&nbsp;created_at: DateTime, <br>&nbsp;updated_at: DateTime, <br>&nbsp;status: int<br>} | FAIL |
|  | *PUT* | ```v1/deparment/{id}``` | _Update A Deparment_ |Authorization Header| OK | FAIL |
|  | *DELETE* | ```v1/deparment/{id}``` | _Delete A Deparment_ |Authorization HeaderAuthorization Header| OK | FAIL |
| USER DEPARTMENT | *POST* | ```v1/user_department/``` | _Create A UserDepartment_ |Authorization Header| OK | FAIL |
|  | *GET* | ```v1/user_department/{id}``` | _Get UserDeparment By ID_ |Authorization Header|  {<br>&nbsp;ud_id : int, <br>&nbsp;user_id: int, <br>&nbsp;department_id: int<br>} | FAIL |
|  | *DELETE* | ```v1/user_department/{id}``` | _Delete A UserDeparment_ |Authorization Header| OK | FAIL |

## AUTH SERVICE
| NAMESPACE | METHOD | ROUTE | FUNCTIONALITY |REQUIRE| RESPONSE SUCCESS | RESPONSE FAIL |
| --------- | -------| ----- | ------------- |---------------- | ---------------- | ------------- |
| AUTH | *POST* | ```v1/auth/signin``` | _SIGN IN_ |None| {<br>&nbsp;access token : String,<br>&nbsp;refresh_token: String<br>&nbsp;}| 
|  | *POST* | ```v1/auth/register``` | _SIGN UP_ |None| OK | FAIL |
|  | *GET* | ```v1/auth/refresh``` | _REFRESH TOKEN_ |Cookies: refresh_token=String| String (new access token) | FAIL |
|  | *DELETE* | ```v1/auth/delete/{token}``` | _DELETE REFRESH TOKEN_ | NONE | String (new access token) | FAIL |

## POST SERVICE
| NAMESPACE | METHOD | ROUTE | FUNCTIONALITY |REQUIRE|
| --------- | -------| ----- | ------------- |---------------- |
| POST | *GET* | ```v1/post``` | _GET ALL POST_ || 
|  | *POST* | ```v1/post``` | _CREATE_ ||
|  | *GET* | ```v1/post/{id}``` | _GET A POST_ ||
|  | *PUT* | ```v1/post/{id}``` | _UPDATE_ ||
| * | *DELETE* | ```v1/post/{id}``` | _DELETE_ ||

## NOTIFICATION SERVICE
| NAMESPACE | METHOD | ROUTE | FUNCTIONALITY |REQUIRE|
| --------- | -------| ----- | ------------- |---------------- |
| NOTIFICATION | *GET* | ```v1/notification``` | _GET ALL NOTIFICATION_ || 

## Note 
- To use `docker-compose` you must fill in POSTGRES_USER, POSTGRES_DB and POSTGRES_PASSWORD in postgres service and DATABASE_URL (like postgres://{db_user_name}:{db_user_password}@db:5432/{db_name} ) in other service 

- API gateway config in api/gate-way/config.toml
