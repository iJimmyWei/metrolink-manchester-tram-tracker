# Manchester Metrolink Tram Tracker
Written in Rust. Maps out every tram operating live in Manchester using the TFGM API ([https://developer.tfgm.com/docs/services](https://developer.tfgm.com/docs/services)) and their position.


## Concept/Theory
Every tram station provides information on what trams will arrive within a specified time. Given such information, and also the knowledge that a tram has actually stopped at the station, we can derive exactly where and when every tram is located on the Metrolink service in Manchester (UK).

![tramDiagram](https://user-images.githubusercontent.com/46089773/69906456-89cdcf00-13bb-11ea-824a-895642002d71.png)

From the above diagram, we can deduce that `Ladywell` was the last position TRAM-ABC had left off at based on the time intervals. By identifying each tram within the Metrolink network, we are able to quickly traverse the whole network and determine what trams are where. Furthermore, a restriction by the API to only provide the next 3 trams coming up can be circumvented by knowing what the next 3 trams for the previous station are (and so on and so forth).
 
 ## Future plans
 Precise tracking of each trams position based off of time elapsed - delta distance difference between 2 tram stops over a unit of time (better predictions if using machine learning)
 
