# Problem definition

Main issue with the queueing in application like this one is requiring two, seemingly countering requirements:

- on one hand, it's required to have automated way to order the queue, so that operator wouldn't need to manually change
  the order all the time
- but on the other - sometimes operator does need to change the order manually, based on unexpected events, how they
  want to change the "flow" etc.

# Manual ordering

Manual ordering of the queue is quite straightforward - let operator change the ordering using appropriate UI controls (
be it drag-and-drop or simple arrows) that would be appropriately reflected on the backend.

This is something that can be quite easily be done by using appropriate order column, with numbers representing
ordering. With some additional number gaps between entries (for example 100 or 1000) its quite easy to insert entries
in-between existing ones wihtout having to parse things again.

But on the flip side - with many entries in the queue, and people ordering multiple songs one after another (or even in
bulk) this would make it quite difficult and time consuming to process. Constant clicking on reordering would make it
quite the hassle to manage.

# Semi-automated ordering

It is possible to semi-automate the ordering of the queue by making ordering based on some additional information.
For example order in which first song was added by given singer.

This however would make it challenging to calculate fairly what would be the ordering of the queue in the long run,
as it would require inserting people that added their first song way later somewhere in the middle of the queue,
to let them sing earlier than after an hour or so.

So it should be quite easy to do for the "first round" so to speak - as long as nobody sung second song (or second set
of songs) we are just adding people at the end of the queue.

This is its own rabbit hole, as we would need to focus on number of songs being sung by each singer and in many cases we
could start with lets say 2-3 songs for each singer,
but after time decrease it to 1 song per singer when there are more people in the queue.

# Fully automated ordering

Fully automated ordering is the most complex one, but also the most useful one.
It would require to have some kind of algorithm that would take into account multiple factors, such as

- how many songs each singer has sung
- when was the first song sung and requested by given singer
- how many songs are to be played in session now

This is however challenging in its own right, as operator MUST be able to overwrite whatever automation gives them. Be
it due to change in flow they want to adjust, or due to issues.

# Roadmap of ordering design

## First - fully manually, on song-by-song level

There would be a lot of clicking or drag-n-dropping, but would make it a
lot easier to created code for that.

## Second - still manually - but on the singer level

This would probably be the greatest jump in terms of usability, as instead of managining 30-40 entries, operator would
need to manage it on level of 6-10 people singing which should be much easier to handle.
On that level it would be also possible to disable someone from singing, or move them quickly after somebody else if
they are unavailable. Still however, it would make it much more rigorous in terms of how to start the session and where
to put the people. As probably depending on the state of the session, it would be possible to differently handle the
order.

Operator would also need to make sure that order is somewhat fair, leaving to them a bunch of work to do.

## Third - semi-automatic on the singer level

Similar to second one, but algorithm for where to put people in the queue.

## Fourth - full(?) automation

Analyze current state of the queue and previous entries. Automatically reorder people.

This should be triggerable by the operator, rather than done all the time. Otherwise it could mess up what have been
manually changed for one reason or another.

## Side note - duets and multiple people singing

This is quite tricky to automate (which is why automation is the last part anyway). Especially in terms of how to treat
this fairly?

On one hand, you can say that for a duet each person gets "half" a song. But thats not exactly the case - as for some
songs one of the singers sings for maybe 10-20% of the song as they are doing mainly backvocals. Which wouldn't be fair
to them to treat as even "half" a song being sung by them.

On the other, it won't be fair to give main singer "full" song each time, as in that case the other person would get to
sing for "free" piggybacking on the other person and then singing themselves, effectively doubling amount of singing for
them.

## Side note - number of songs per singer changes

This one is tricky in its own right. That would certainly force recalculations of all automated queue entries (not
manual ones at least).

With automation that would be even harder to do, as recalculation would need to take effect for all the unplayed queue
entries.

# Implementations

## Fully manually - queue entry level

Easiest of the bunch - buttons up/down + backend function to move entries. No need to handle how many songs each person
sung. No need to handle fair inserting.

## Fully manually - on singer level

We are still leaving what is being done on queue entry level. However we are also adding manual change of ordering of
singers for given session.
This would require:

- similar logic for changing singers, as for queue entries themselves
- on that order change - appropriate changes in queue entries (as some sort of trigger)
- to limit amount of manipulations on database - it should be triggered by additional button, rather than after each
  change of entry
