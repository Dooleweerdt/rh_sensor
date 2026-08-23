#include "zephyr/kernel.h"
#include <stdint.h>
#include <stdio.h>
#include <zephyr/net/mqtt.h>
#include <zephyr/net/socket.h>
#include <zephyr/logging/log.h>
#include <zephyr/posix/poll.h>
#include <zephyr/random/random.h>

#include <app/wifi_settings.h>

LOG_MODULE_REGISTER(mqtt_bridge, LOG_LEVEL_INF);

#define COMM_TIMEOUT 2000
#define PUBLISH_TIMEOUT 500

#ifndef WIFI_CREDENTIALS
#define MQTT_USER "Dummy"
#define MQTT_PASSWORD "Pass"
#endif

static struct mqtt_client client_ctx;
#if CONFIG_WIFI
static struct sockaddr_storage broker_addr;
#endif
static uint8_t rx_buffer[256];
static uint8_t tx_buffer[256];

static struct pollfd fds[1];
static int nfds;
static bool is_connected = false;

static void mqtt_evt_handler(struct mqtt_client *const client, const struct mqtt_evt *evt)
{
    switch (evt->type) {
        case MQTT_EVT_CONNACK:
            if (evt->result == 0) {
                LOG_INF("MQTT client connected!\n");
                is_connected = true;
            } else if (evt->result == MQTT_NOT_AUTHORIZED) {
                LOG_INF("MQTT Authorization failed!");
            } else {
                LOG_INF("MQTT connection failed: %d\n", evt->result);
            }
        break;
        
        default:
        break;
    }
}

int32_t mqtt_bridge_connect(void) {
    mqtt_client_init(&client_ctx);

    #if CONFIG_WIFI
    // Configure broker address details (e.g., 192.168.1.21:1883)
    struct sockaddr_in *broker4 = (struct sockaddr_in *)&broker_addr;
    broker4->sin_family = AF_INET;
    broker4->sin_port = htons(1883);
    zsock_inet_pton(AF_INET, "192.168.1.21", &broker4->sin_addr);
    #endif

    client_ctx.client_id.utf8 = (const uint8_t *)"Zephyr_RH_Sensor";
    client_ctx.client_id.size = strlen("Zephyr_RH_Sensor");
    #if CONFIG_WIFI
    client_ctx.broker = &broker_addr;
    #endif
    client_ctx.evt_cb = mqtt_evt_handler;

    client_ctx.user_name = malloc(sizeof(struct mqtt_utf8));   
    client_ctx.user_name->utf8 = (const uint8_t*)MQTT_USER;
    client_ctx.user_name->size = strlen(MQTT_USER);
    client_ctx.password = malloc(sizeof(struct mqtt_utf8));
    client_ctx.password->utf8 = (const uint8_t*)MQTT_PASSWORD;
    client_ctx.password->size = strlen(MQTT_PASSWORD);
    client_ctx.transport.type = MQTT_TRANSPORT_NON_SECURE;
    client_ctx.protocol_version = MQTT_VERSION_3_1_1;

    client_ctx.rx_buf = rx_buffer;
    client_ctx.rx_buf_size = sizeof(rx_buffer);
    client_ctx.tx_buf = tx_buffer;
    client_ctx.tx_buf_size = sizeof(tx_buffer);

    int err = -EINVAL;
    int i = 3;

    while ((i > 0) && (is_connected == false)) {
        err = mqtt_connect(&client_ctx);
        if (err != 0) {
            k_sleep(K_SECONDS(1));
            continue;
        }

        if (client_ctx.transport.type == MQTT_TRANSPORT_NON_SECURE) {
            fds[0].fd = client_ctx.transport.tcp.sock;
        }

        fds[0].events = POLLIN;
        nfds = 1;

        if (nfds > 0) {
            if (poll(fds, nfds, COMM_TIMEOUT) < 0) {
                LOG_ERR("poll error: %d", errno);
            }
            else {
                mqtt_input(&client_ctx);
            }
        }

        if (is_connected == false) {
            mqtt_abort(&client_ctx);
            k_sleep(K_SECONDS(1));
        }

        LOG_INF("mqtt_bridge_connect: mqtt_connect status=%d, cnt=%d",err, i);
    }

    return err;
}

int32_t mqtt_bridge_send_data(const uint8_t * payload, size_t len) {
    struct mqtt_publish_param param;

    param.message.topic.qos = MQTT_QOS_0_AT_MOST_ONCE;
    param.message.topic.topic.utf8 = (uint8_t*)"RH4sense";
    param.message.topic.topic.size = strlen("RH4sense");
    param.message.payload.data = (uint8_t*)payload;
    param.message.payload.len = len;
    param.message_id = sys_rand16_get();
    
    param.dup_flag = 0;
    param.retain_flag = 0;

    int32_t ret = mqtt_publish(&client_ctx, &param);

    if (ret != 0) {
        LOG_ERR("mqtt_bridge_send_data: publish error = %d", ret);
        return ret;
    }

    if (nfds > 0) {
        ret = poll(fds, nfds, PUBLISH_TIMEOUT);
        if (ret < 0) {
            LOG_ERR("mqtt_bridge_send_data: poll error: %d", errno);
            return ret;
        }
    }

    ret = mqtt_input(&client_ctx);

    if (ret != 0) {
        LOG_ERR("mqtt_bridge_send_data: input error %d", ret); 
        return ret;
    }

    ret = mqtt_live(&client_ctx);

    if ((ret != 0) && (ret != -EAGAIN)) {
        LOG_ERR("mqtt_bridge_send_data: live error %d", ret);
        return ret;
    }

    ret = mqtt_input(&client_ctx);

    return ret;
}
