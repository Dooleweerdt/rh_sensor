#include "zephyr/net/net_mgmt.h"
#include <string.h>
#include <zephyr/kernel.h>
#include <zephyr/net/wifi_mgmt.h>
#include <zephyr/net/net_config.h>

// Wifi code from Shawn Hymel (DigiKey): https://www.digikey.dk/da/maker/tutorials/2025/introduction-to-zephyr-part-11-wifi-and-iot 

LOG_MODULE_REGISTER(wifi_app, LOG_LEVEL_INF);

// Event callbacks
static struct net_mgmt_event_callback wifi_cb;
static struct net_mgmt_event_callback ipv4_cb;

// Semaphores
static K_SEM_DEFINE(sem_wifi, 0, 1);
static K_SEM_DEFINE(sem_ipv4, 0, 1);

// Called when the WiFi is connected
static void on_wifi_connection_event(struct net_mgmt_event_callback *cb,
                                     uint64_t mgmt_event,
                                     struct net_if *iface)
{
    const struct wifi_status *status = (const struct wifi_status *)cb->info;

    if (mgmt_event == NET_EVENT_WIFI_CONNECT_RESULT) {
        if (status->status) {
            LOG_INF("Error (%d): Connection request failed\r\n", status->status);
        } else {
            LOG_INF("Connected!\r\n");
            k_sem_give(&sem_wifi);
        }
    } else if (mgmt_event == NET_EVENT_WIFI_DISCONNECT_RESULT) {
        if (status->status) {
            LOG_INF("Error (%d): Disconnection request failed\r\n", status->status);
        } else {
            LOG_INF("Disconnected\r\n");
        }
    }
}

// Event handler for WiFi management events
static void on_ipv4_obtained(struct net_mgmt_event_callback *cb,
                             uint64_t mgmt_event,
                             struct net_if *iface)
{
    // Signal that the IP address has been obtained
    if (mgmt_event == NET_EVENT_IPV4_ADDR_ADD) {
        LOG_INF("IPv4 Address obtained\r\n");
        k_sem_give(&sem_ipv4);
    }
}

// Initialize the WiFi event callbacks
void wifi_init(void)
{
    static bool is_initialized = false;
    
    if (is_initialized) {
        LOG_WRN("Wi-Fi Subsystems already initialized. Skipping.");
        return;
    }

    LOG_INF("Configuring Wi-Fi Subsystems...");
    
    // 1. Get a handle to Zephyr's default network interface (the ESP32 Wi-Fi station)
    struct net_if *iface = net_if_get_default();
    if (!iface) {
        LOG_ERR("Error: No network interface found!");
        return;
    }

    // 2. Administratively bring the interface UP (turns on the network stack for this card)
    net_if_up(iface);

    // Initialize the event callbacks
    net_mgmt_init_event_callback(&wifi_cb,
                                 on_wifi_connection_event,
                                 NET_EVENT_WIFI_CONNECT_RESULT | NET_EVENT_WIFI_DISCONNECT_RESULT);
    net_mgmt_init_event_callback(&ipv4_cb,
                                 on_ipv4_obtained,
                                 NET_EVENT_IPV4_ADDR_ADD);

    // Add the event callbacks
    net_mgmt_add_event_callback(&wifi_cb);
    net_mgmt_add_event_callback(&ipv4_cb);

    is_initialized = true;

    /* Wait for WiFi to initialize */
    k_sleep(K_MSEC(500));
}

// Connect to the WiFi network (blocking)
int wifi_connect(char *ssid, char *psk)
{
    int ret;
    struct net_if *iface;
    static struct wifi_connect_req_params params;

    // Get the default networking interface
    iface = net_if_get_default();

    // Fill in the connection request parameters
    params.ssid = (const uint8_t *)ssid;
    params.ssid_length = strlen(ssid);
    params.psk = (const uint8_t *)psk;
    params.psk_length = strlen(psk);
    params.security = WIFI_SECURITY_TYPE_PSK;
    params.band = WIFI_FREQ_BAND_UNKNOWN;
    params.channel = WIFI_CHANNEL_ANY;
    params.mfp = WIFI_MFP_OPTIONAL;

    // Connect to the WiFi network
    ret = net_mgmt(NET_REQUEST_WIFI_CONNECT,
                   iface,
                   &params,
                   sizeof(params));

    LOG_INF("WIFI Driver: Waiting for Semaphor!");

    // Wait for the connection to complete
    ret = k_sem_take(&sem_wifi, K_MSEC(10000));
    if (ret == -EAGAIN) {
        LOG_ERR("WIFI Driver: Connection TIMEOUT! The driver/hardware hung up.");
        return -ETIMEDOUT;
    } else {
        LOG_INF("WIFI Driver: Semaphor acquired!");
    }
    return ret;
}

// Wait for IP address (blocking)
void wifi_wait_for_ip_addr(void)
{
    struct wifi_iface_status status;
    struct net_if *iface;
    char ip_addr[NET_IPV4_ADDR_LEN];
    char gw_addr[NET_IPV4_ADDR_LEN];

    // Get interface
    iface = net_if_get_default();

    // Wait for the IPv4 address to be obtained
    int ret = k_sem_take(&sem_ipv4, K_MSEC(10000));

    if (ret == -EAGAIN) {
        LOG_ERR("WIFI Driver: Wait for IP TIMEOUT! The driver/hardware hung up.");
        return;
    }

    // Get the WiFi status
    if (net_mgmt(NET_REQUEST_WIFI_IFACE_STATUS,
                 iface,
                 &status,
                 sizeof(struct wifi_iface_status))) {
        LOG_ERR("Error: WiFi status request failed!");
    }

    // Get the IP address
    memset(ip_addr, 0, sizeof(ip_addr));
    if (net_addr_ntop(AF_INET,
                      &iface->config.ip.ipv4->unicast[0].ipv4.address.in_addr,
                      ip_addr,
                      sizeof(ip_addr)) == NULL) {
        LOG_ERR("Error: Could not convert IP address to string!");
    }

    // Get the gateway address
    memset(gw_addr, 0, sizeof(gw_addr));
    if (net_addr_ntop(AF_INET,
                      &iface->config.ip.ipv4->gw,
                      gw_addr,
                      sizeof(gw_addr)) == NULL) {
        LOG_ERR("Error: Could not convert gateway address to string!");
    }

    // Print the WiFi status
    LOG_INF("WiFi status:");
    if (status.state >= WIFI_STATE_ASSOCIATED) {
       LOG_INF("  SSID: %-32s", status.ssid);
       LOG_INF("  Band: %s", wifi_band_txt(status.band));
       LOG_INF("  Channel: %d", status.channel);
       LOG_INF("  Security: %s", wifi_security_txt(status.security));
       LOG_INF("  IP address: %s", ip_addr);
       LOG_INF("  Gateway: %s", gw_addr);
    }

    /* Small delay to ensure network stack is ready */
    k_sleep(K_MSEC(500));
}

// Disconnect from the WiFi network
int wifi_disconnect(void)
{
    int ret;
    struct net_if *iface = net_if_get_default();

    ret = net_mgmt(NET_REQUEST_WIFI_DISCONNECT, iface, NULL, 0);

    return ret;
}
