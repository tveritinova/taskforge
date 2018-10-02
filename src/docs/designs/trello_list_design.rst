Trello backed List
==============================

This document describes the design of Trello backed List for saving and loading tasks from Trello.

Design
------

Authorization
+++++++++++++

Via ``task trello auth`` you can get a link to grant taskforge access to your Trello account and data.

Choosing board
++++++++++++++

You must specify the board with ``task trello set-board <board_id>``

Implementation of base class List functionality
++++++++++++++++++++++++++++++++++++++++++++++++

``search`` -- searching in all lists of board except "done" list of Trello board

``add`` -- add task to "taskforge" list of Trello board

``list`` -- all tasks in all lists of board except "done" list of Trello board

``find_by_id`` -- Find a task by id

``current`` -- the oldest uncompleted task in all lists of board except "done" list of Trello 

``complete`` -- move task to "done" list of Trello board

``update`` -- update task by id

``add_note`` -- add note to a task by id