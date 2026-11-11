/*
 * Copyright 2026 Octavian Ilie
 * SPDX-License-Identifier: Apache-2.0
 *
 * Project Zapp: stand-in for the reference voice codecs.
 *
 * The TIA/3GPP2/Qualcomm reference codec sources that 1xBTS compiles from
 * crates/cdma-voice/csrc are not distributed with Project Zapp because their
 * redistribution terms are unresolved (see SOURCE_ORIGIN.md). When csrc/ is
 * absent, build.rs links this file instead. Every initialiser returns NULL,
 * so each Rust codec constructor returns an error and no codec can be
 * instantiated. The remaining functions are never reached through the safe
 * Rust API; they fail defensively if called.
 */

#include <stddef.h>
#include <stdint.h>

/* EVRC (IS-127), SO 3 */
void *evrc_encoder_init(int16_t min_rate, int16_t max_rate, int16_t noise_suppression)
{
    (void)min_rate; (void)max_rate; (void)noise_suppression;
    return NULL;
}
void evrc_encoder_uninit(void *c) { (void)c; }
int32_t evrc_encoder_encode_to_packet(void *c, int16_t *speech, size_t speech_samples,
                                      uint8_t *packet, size_t packet_max_bytes)
{
    (void)c; (void)speech; (void)speech_samples; (void)packet; (void)packet_max_bytes;
    return -1;
}
void *evrc_decoder_init(void) { return NULL; }
void evrc_decoder_uninit(void *c) { (void)c; }
int32_t evrc_decoder_decode_from_packet(void *c, const uint8_t *packet, size_t packet_bytes,
                                        int16_t *speech, size_t speech_max_samples)
{
    (void)c; (void)packet; (void)packet_bytes; (void)speech; (void)speech_max_samples;
    return -1;
}
int32_t evrc_decoder_decode_erasure(void *c, int16_t *speech, size_t speech_max_samples)
{
    (void)c; (void)speech; (void)speech_max_samples;
    return -1;
}

/* EVRC-B (SO 68) and EVRC-WB (SO 70) */
void *evrcbw_encoder_init(int32_t mode) { (void)mode; return NULL; }
void *evrcbw_encoder_init_with_operating_point(int32_t mode, int32_t operating_point)
{
    (void)mode; (void)operating_point;
    return NULL;
}
void evrcbw_encoder_uninit(void *c) { (void)c; }
int32_t evrcbw_encoder_encode_to_words(void *c, const int16_t *speech, size_t speech_samples,
                                       int16_t *rate, int16_t *words, size_t words_capacity)
{
    (void)c; (void)speech; (void)speech_samples; (void)rate; (void)words; (void)words_capacity;
    return -1;
}
void *evrcbw_decoder_init(int32_t mode) { (void)mode; return NULL; }
void evrcbw_decoder_uninit(void *c) { (void)c; }
int32_t evrcbw_decoder_decode_from_words(void *c, int16_t rate, const int16_t *words,
                                         size_t words_count, int16_t *speech,
                                         size_t speech_max_samples)
{
    (void)c; (void)rate; (void)words; (void)words_count; (void)speech; (void)speech_max_samples;
    return -1;
}

/* QCELP-13K (TIA/ANSI-733), SO 32768 */
void *qcelp13k_encoder_init(int32_t max_rate, int32_t min_rate)
{
    (void)max_rate; (void)min_rate;
    return NULL;
}
void qcelp13k_encoder_uninit(void *ctx) { (void)ctx; }
int32_t qcelp13k_encoder_encode_to_packet(void *ctx, const int16_t *speech, size_t samples,
                                          uint8_t *packet, size_t max_bytes)
{
    (void)ctx; (void)speech; (void)samples; (void)packet; (void)max_bytes;
    return -1;
}
void *qcelp13k_decoder_init(void) { return NULL; }
void qcelp13k_decoder_uninit(void *ctx) { (void)ctx; }
int32_t qcelp13k_decoder_decode_from_packet(void *ctx, const uint8_t *packet, size_t bytes,
                                            int16_t *speech, size_t max_samples)
{
    (void)ctx; (void)packet; (void)bytes; (void)speech; (void)max_samples;
    return -1;
}

/* TIA-96 / IS-96A, SO 1 */
void *tia96_encoder_init(void) { return NULL; }
void tia96_encoder_uninit(void *ctx) { (void)ctx; }
int32_t tia96_encoder_encode_to_packet(void *ctx, const int16_t *speech, size_t samples,
                                       uint8_t *packet, size_t max_bytes)
{
    (void)ctx; (void)speech; (void)samples; (void)packet; (void)max_bytes;
    return -1;
}
void *tia96_decoder_init(void) { return NULL; }
void tia96_decoder_uninit(void *ctx) { (void)ctx; }
int32_t tia96_decoder_decode_from_packet(void *ctx, const uint8_t *packet, size_t bytes,
                                         int16_t *speech, size_t max_samples)
{
    (void)ctx; (void)packet; (void)bytes; (void)speech; (void)max_samples;
    return -1;
}
