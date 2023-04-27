//
// Copyright (c) 2023 ZettaScale Technology
//
// This program and the accompanying materials are made available under the
// terms of the Eclipse Public License 2.0 which is available at
// http://www.eclipse.org/legal/epl-2.0, or the Apache License, Version 2.0
// which is available at https://www.apache.org/licenses/LICENSE-2.0.
//
// SPDX-License-Identifier: EPL-2.0 OR Apache-2.0
//
// Contributors:
//   ZettaScale Zenoh Team, <zenoh@zettascale.tech>

#include <stddef.h>
#include <stdio.h>
#include <string.h>

#undef NDEBUG
#include <assert.h>

#include "zenoh.h"

// include sleep
#include <unistd.h>

const char* keyexpr = "demo/example/session_move_test";
const char* value = "Session Move!";

int main(int argc, char** argv) {
    z_owned_config_t config = z_config_default();
    z_owned_session_t s = z_open(z_move(config));
    z_owned_publisher_t pub = z_declare_publisher(z_loan(s), z_keyexpr(keyexpr), NULL);
    z_publisher_put_options_t options = z_publisher_put_options_default();
    options.encoding = z_encoding(Z_ENCODING_PREFIX_TEXT_PLAIN, NULL);
    z_publisher_put(z_loan(pub), (const uint8_t*)value, strlen(value), &options);
    sleep(1);
    z_undeclare_publisher(z_move(pub));
    z_close(z_move(s));
}