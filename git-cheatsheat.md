# Setting up a git repository
There are two main ways that I start working on a project with git:
1. Start from scratch by creating a new repository locally and pushing it to github
2. Start by cloning an existing repository from github

## Create a new repository
Create a new folder and cd into it:
```bash
cd <path-to-folder>
```
then run `git init` to initialize a new git repository
```bash
git init
```
Go ahead and create a new repository on github and copy the git url. Then you can run `git remote add origin <git-url>` to add the remote repository to your local repository:
```bash
git remote add origin <git-url>
```

## Clone an existing repository from github
Use the `git clone` command to clone a repository from github:
```bash
git clone <git URL>
```
**That's it!** you do not need use `git init` becasause cloning a repository will create a new directory for you with all the git configuration done.

# Using git
Now that your git repository is setup, you can start using git. here are a couple of commands to get you started:

| Command | Description |
| --- | --- |
| `git status` | shows the status of your local repository |
| `git add .` | adds new files to the staging area (you can replace the "." with the path to a specific file if you just want to add one file) |
| `git commit -m "<description>"` | commits the staged files to the local repository |
| `git push origin master` | pushes the local repository to the remote repository. if you are working on a branch other than master, replace "master" with the branch name. |
| `git pull origin master` | pulls the remote repository to the local repository. if you are working on a branch other than master, replace "master" with the branch name |

> **My daily workflow looks like this:**
> 1. I use one of the methods at the top to setup my git repository.
> 2. then I start working on my project, adding new files, writing code, etc.
> 3. when I'm done, I run `git add .` to add all the new files to the staging area.
> 4. then I run `git commit -m "<description>"` to commit the new files to the local repository.
> 5. finally I run `git push origin master` to push the local repository to the remote repository.

If I switch to a new computer, I'll either use `git clone` to download the repository from github or `git pull` to update the local repository if I already had cloned it previously and just want to update it with my changes.