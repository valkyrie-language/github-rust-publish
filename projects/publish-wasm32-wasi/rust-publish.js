#!/usr/bin/env node

import {getInput} from '@actions/core';
import {run} from "./src/index.js";

const name = getInput('name');

run.run()