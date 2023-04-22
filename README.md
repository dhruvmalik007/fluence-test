## Tutorial  of Aqua/Marine course:

1. Aqua: DSL for developing the application in order to  deploy the distributed execution without centralized co-ordination.  they are not running in the backend as the contenarized applications. 


2. Marine: its the sdk written in rus, which defines the internal runti;e enviornment which will be executing the operations . thus all the microservices that needed to deploy (for instance ML model or the web app) will be defined  via modules which are then added in the `fluence.yaml` file. this will be including the