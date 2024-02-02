---
title: Exploring Autocomplete
sub_title: A brief look at data structures
author: Presentation by Joey Malvinni
theme: 
    path: theme.yaml 
---

# Brief Introduction
---

<!-- column_layout: [2, 1] -->

<!-- column: 1 -->
![](media/autocomplete_in_vscode.png)

<!-- column: 0 -->
* What is autocomplete?
* History of autocomplete
* Modern uses of autocomplete

<!-- end_slide -->

# Overview
---

Implement autocomplete:
* Naive approach
* Tries
* Radix Tries

Final results:
* Speed comparison

<!-- end_slide -->

# Getting started

Template Github repository for you to clone, available at `https://github.com/DPHSCodingClub/autocomplete_lab`

* Clone the repository
* Install node requirements
* Run server using `npm start`

<!-- end_slide -->

# Naive Approach

* Loop through every word
* Check if it starts with a prefix
* Add matches to final length


<!-- end_slide -->
