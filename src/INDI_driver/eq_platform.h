#pragma once

#include "defaultdevice.h"
#include "indiguiderinterface.h"

namespace Connection { class Serial; }

class EQController : public INDI::DefaultDevice, public INDI::GuiderInterface {
    public:
        EQController() = default;

        virtual bool initProperties() override;
        virtual bool updateProperties() override;

        virtual bool ISNewNumber(const char *dev, const char *name, double values[], char *names[], int n) override;

    protected:
        virtual IPState GuideNorth(uint32_t ms) override;
        virtual IPState GuideSouth(uint32_t ms) override;
        virtual IPState GuideEast(uint32_t ms) override;
        virtual IPState GuideWest(uint32_t ms) override;

        bool Connect();
        bool Disconnect();
        const char *getDefaultName();

    private:
        Connection::Serial *serialConnection = NULL;
        bool Handshake();
        int PortFD;
};
