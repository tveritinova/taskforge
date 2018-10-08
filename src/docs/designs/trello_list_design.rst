Trello backed List
==============================

This document describes the design of Trello backed List for saving and loading tasks from Trello.

Design
------

Configurration
++++++++++++++

You must put your ``trello-token`` and ``board-id`` to the configation file in ``[list.config]`` section. If board isn't specified, the board "Taskforge" will be created.



Implementation of base class List functionality
++++++++++++++++++++++++++++++++++++++++++++++++

``search`` -- searching in all lists of board except "done" list of Trello board

``add`` -- add task to "taskforge" list of Trello board

``list`` -- all tasks in all lists of board except "done" list of Trello board

``find_by_id`` -- Find a task by id

``current`` -- the oldest uncompleted task in all lists of board except "done" list of Trello 

``complete``:

Completed tasks are holding in "done" list in booard. You can configure list name with ``done-list-name`` in ``[list.config]`` section of the configuration file or you can specify flag ``archive-done-tasks = 1`` to archive completed tasks instead of moving in to "done" list. ``archive-done-tasks`` has more priority then ``done-list-name``.

``update`` -- update task by id

``add_note`` -- add note to a task by id



Dealing with notes
++++++++++++++++++

Notes are mapping to comments. Trello's author are added in the begging of note, like that: ``@username: text``. If note's author is you, there will be no ``@username`` in note text.



Other metadata
++++++++++++++

-  Priority is using for sorting cards in list: the biggest priority is for the first card in list.

-  Context will map to comment with ``#context`` in the begging of it

-  Completed date is the moment when card was moved to the "done" list or archived, depending on your config
