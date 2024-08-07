# What do I actually want?

EDIT: Can check out redash (redash.io). Try connecting to it and see what it is capable of. Maybe that's what I want. If not, then I guess I need to go full custom. If yes, then it's either good enough altogether or just as an advanced mode.

I am thinking of a web page, where you can enter and pick a few filters (provide them with values from the db) you want to use or introduce a query, then get the data and display visualizations.
Basic vizualizations I am thinking of are:
1. route pyramid
2. timeline (e.g. breaking into new grade)


# So what do I do now?

PREP backend for deployment:
    * DONE have envs extracted into a non-public env-file:
        * DONE have config for dev/prod somehow 
    * DONE  move redash config in under the repo, spin up redash to be able to redirect to it from the frontend app (or if it's really hard to even start redash (reinstall compose first) locally, then set it up on some server)
    * DONE: setup redash - need a setup script that generates env and creates postgres-data + could also run the create_db function
        * try creating a custom network (same as on the other machine, we'll need that anyway for the mongo to run)
    * DONE extract envs from composes
    * DONE image definition for the backend app
        * DONE make the sidexporter wihin the container work

PREP frontend for deployment:
    * learn how to and try to deploy it
    * extract env variables per server
    * create readme instructions
    * package in docker
    * when on the server - have mongo user with a password
* get a hold of some server and experiment with deploying.  Expose the beta somewhere
* style the frontend and explain the  page properly

* test deploy

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
* extend app with bouldering

* (from sveltekit node adapter docs) You will typically want to compress responses coming from the server. If you are already deploying your server behind a reverse proxy for SSL or load balancing, it typically results in better performance to also handle compression at that layer since Node.js is single-threaded.

### DONE 

* Have a local DB that'd save the scraped results (mongo?)
    * if we have mongo, then store all ascents in one collection with index on user (and secondary index on date maybe if we'll be querying by that?)
* spin up redash and connect it to the db, load db with actual ascents, see what we can get from redash. Maybe it's all we need? If not:


# Notes

## Redash issues

### Postgres service network availability

I was having a hard time setting up redash on my arch machine.
Redash services could not connect to the postgres service defined in compose. 
They would not resolve the container's ip.

In fact, the container was not getting any ip (why, I have no idea).

What finally worked was running postgres in a separate compose with a bridge network defined in there,
and then running redash separately connecting to the same network (with external: true, of course)

Can still try figuring out if it's possible to put that all in one compose file, but if not - hey, it works.
And it might be able to work in 1 compose in production, the split is more for the dev environment anyway.

Oh well, it works now even in 1 compose. 
