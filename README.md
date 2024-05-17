Sooo, it turns out the API is available and I just need:

* to provide some browser user-agent in the request
* have the session cookie (in connect.sid cookie)

So I'll need to execute login through automation like selenium, but then I can have the connect.sid cookie and just call requests normally. 
Could try using [puppteer](https://pptr.dev/) for that.

Then what do I do?

* Have a local DB that'd save the scraped results (mongo?)
* Display last few ascents and reload only on request 
    * Only start webdriver on 401/403
* CORE Idea - display the pyramid and allow query'ing. Maybe later have some set queries already like:
    * a pyarmid from:to dates
    * a pyramid from a given crag/area only
* compare pyramids
