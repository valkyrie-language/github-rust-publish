#!/usr/bin/env node

import {getInput} from '@actions/core';
import {runWithConfig} from "./src/index.js";

runWithConfig(getInput('config'), getInput('mode'))