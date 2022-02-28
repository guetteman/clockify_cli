# Clockify CLI

This is a CLI to print monthly reports from Clockify.

Make sure you have the following variables exported in your bash
(usually putting in a place like `~/.bash_profile` does the job):

```
export CLOCKIFY_API_KEY=
export CLOCKIFY_WORKSPACE_ID=
```

Download the binary file from
[here](https://github.com/guetteman/clockify_cli/tree/main/dist/clockify)

## List tasks

This command lists the tasks for the provided month:

```
clockify list-tasks FEB
```

## Timesheet

This command shows how much time you worked in a project for the provided month.
It also shows the MPB based on the "working days" parameter:

```
clockify timesheet FEB 20
```
