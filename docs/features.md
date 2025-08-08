# Features

## Wording

MUST - very important requirement for things to be usable
SHOULD - important, but not very important
MAY - nice to have, but possible to overcome by other features or usability is limited due to low frequency of usage

~~Strukthrough entries~~ are implemented

## Operator-side

Operator MUST be required to login to access its functionality.

Operator page MUST be separate from normal page - so just in case they can also add people themselves just like the normal users

Operator MUST be able to start and end session

Operator MUST have a way to "merge" multiple people into one, to prevent (accidental or not) possibility of people jumping queue - or by moving song to somebody else

Operator MUST have a way to edit given queue entry - be it by changing who is singing or by changing song on the fly

Operator MUST have a way to change queue order even after automation takes place - in case given person is not available

Operator MUST have a way to disable given person - in case they leave the venue

Operator SHOULD have a way to show with external popup current queue status

Operator SHOULD have a way to specify number of songs sung by given person

Operator SHOULD have a way to specify how many people are being shown in queue status popup

## User-side

~~User MUST be able to easily select song they want to sing~~

~~User MUST be able to specify name of the second person~~

~~User MUST be able to specify some notes about their request, so the operator could see them (change in pitch, back melody etc.)~~

~~User SHOULD be able to specify their own songs, in case it is not available in local database (which database then saves for future use)~~

User SHOULD be able to check behavior of the app in clear and concise language, to reduce any kind of confusion

User SHOULD be able to add multiple songs at once

User MAY be able to submit their feedback as a simple form for creator of the app to then review

User MAY be able to add multiple songs in one go, as a "shopping cart" type of thing to then be able to send all of them at once

## Admin-side

Admin SHOULD be able to do the same things as operator

Admin MUST be able to review feedback for the app

## Deployment

App MUST have appropriate Dockerfile and docker-compose that makes it possible to deploy

Docker compose file MAY have additional nginx component

## General info

App MUST handle starting and ending session - when ending session it should clear queue from entries that were unused

App MUST be bi-lingual - i.e. interface should be available in Polish and English

App SHOULD both detect default language and save language selection if user did it

App MAY show operator QR code to show to users

App MAY show links to other webpages to let users and operators known about author and websites/programs they work with

App MAY require some code to access the UI to prevent outside ingerention - such as `?key=X` parameter in URL that would be provided
