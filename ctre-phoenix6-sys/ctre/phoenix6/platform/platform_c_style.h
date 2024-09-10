/*
 * Copyright (C) Cross The Road Electronics.  All rights reserved.
 * License information can be found in CTRE_LICENSE.txt
 * For support and suggestions contact support@ctr-electronics.com or file
 * an issue tracker at https://github.com/CrossTheRoadElec/Phoenix-Releases
 */
#pragma once

#include "ctre/phoenix/export.h"
#include "stdint.h"
#include "stdbool.h"

CTREXPORT void c_ctre_phoenix6_platform_canbus_sendmessage(uint32_t messageId, uint8_t const *data, uint8_t dataSize, char const *canbus, bool printErr);
CTREXPORT int32_t c_ctre_phoenix6_platform_canbus_receivemessage(uint32_t messageId, uint8_t *data, uint8_t * dataSize, char const *canbus, bool printErr);
CTREXPORT int32_t c_ctre_phoenix6_platform_set_logger_path(char const *path);
CTREXPORT int32_t c_ctre_phoenix6_platform_start_logger(void);
CTREXPORT int32_t c_ctre_phoenix6_platform_stop_logger(void);
CTREXPORT int32_t c_ctre_phoenix6_platform_enable_auto_logging(bool enable);
CTREXPORT int32_t c_ctre_phoenix6_platform_write_raw(char const *name, uint8_t const *data, uint8_t size);
CTREXPORT int32_t c_ctre_phoenix6_platform_write_boolean(char const *name, bool value);
CTREXPORT int32_t c_ctre_phoenix6_platform_write_integer(char const *name, int64_t value, char const *units);
CTREXPORT int32_t c_ctre_phoenix6_platform_write_float(char const *name, float value, char const *units);
CTREXPORT int32_t c_ctre_phoenix6_platform_write_double(char const *name, double value, char const *units);
CTREXPORT int32_t c_ctre_phoenix6_platform_write_string(char const *name, char const *value);
CTREXPORT int32_t c_ctre_phoenix6_platform_write_boolean_array(char const *name, bool const *values, uint8_t count);
CTREXPORT int32_t c_ctre_phoenix6_platform_write_integer_array(char const *name, int64_t const *values, uint8_t count, char const *units);
CTREXPORT int32_t c_ctre_phoenix6_platform_write_float_array(char const *name, float const *values, uint8_t count, char const *units);
CTREXPORT int32_t c_ctre_phoenix6_platform_write_double_array(char const *name, double const *values, uint8_t count, char const *units);
