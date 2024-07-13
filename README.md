# What do I actually want?

EDIT: Can check out redash (redash.io). Try connecting to it and see what it is capable of. Maybe that's what I want. If not, then I guess I need to go full custom. If yes, then it's either good enough altogether or just as an advanced mode.
I need to setup mongoDB and data collection first anyway.

I am thinking of a web page, where you can enter and pick a few filters (provide them with values from the db) you want to use or introduce a query, then get the data and display visualizations.
Basic vizualizations I am thinking of are:
1. route pyramid
2. timeline (e.g. breaking into new grade)

## How do I do that?

    Hmm. I'll still look a bit into a framework that could maybe do that. 
    Otherwise I can supply some filters + examples of the common filters, and for pro users allow them to query the db directly as I don't have any data that would not be accessible otherwise anyway. It's all public. So just make sure there is no "break out of db" query.

# So what do I do now?

* spin up redash and connect it to the db, load db with actual ascents, see what we can get from redash. Maybe it's all we need? If not:
* then think about the api and the webpage that I'd like, sth like:
    * Display last few ascents and reload only on request (user can have a button to load new ascent)
        * Only start webdriver on 401/403
    * CORE Idea - display the pyramid and allow query'ing. Maybe later have some set queries already like:
        * a pyarmid from:to dates
        * a pyramid from a given crag/area only
    * compare pyramids (at first can just have multiple tabs open)
* move redash in under the repo
* make one-command deployment or sth
* have envs extracted into a non-public env-file. Things like:
	* mongo connection string
	* username, password for running sidexporter

### DONE 

* Have a local DB that'd save the scraped results (mongo?)
    * if we have mongo, then store all ascents in one collection with index on user (and secondary index on date maybe if we'll be querying by that?)
