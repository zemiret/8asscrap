# What do I actually want?

EDIT: Can check out redash (redash.io). Try connecting to it and see what it is capable of. Maybe that's what I want. If not, then I guess I need to go full custom. If yes, then it's either good enough altogether or just as an advanced mode.

I am thinking of a web page, where you can enter and pick a few filters (provide them with values from the db) you want to use or introduce a query, then get the data and display visualizations.
Basic vizualizations I am thinking of are:
1. route pyramid
2. timeline (e.g. breaking into new grade)

# So what do I do now?

* move redash config in under the repo, spin up redash to be able to redirect to it from the frontend app (or if it's really hard to even start redash (reinstall compose first) locally, then set it up on some server)
* Write composes to deploy the thing.
* have envs extracted into a non-public env-file. Things like:
	* mongo connection string
	* username, password for running sidexporter
* get a hold of some server and experiment with deploying.  Expose the beta somewhere
* style the frontend and explain the  page properly

* WIP FRONTEND idea - page with steps:

1. Paste the logbook URL
2. Display last ascents I have. Are these not last or there is none? 
    Button to reload ascents.
3. | Basic mode | Advanced mode |

Basic mode (visually):

- list      |    chart_type
- of        |    fields you can filter by (filled automatically if example is clicked)
- examples  |      < CHART HERE >

Advanced mode:

Explain what is redash, what's under the hood, what user can do, blablabla
<Button to redash instance>

* consider if I need any form of auth for the API

### DONE 

* Have a local DB that'd save the scraped results (mongo?)
    * if we have mongo, then store all ascents in one collection with index on user (and secondary index on date maybe if we'll be querying by that?)
* spin up redash and connect it to the db, load db with actual ascents, see what we can get from redash. Maybe it's all we need? If not:
