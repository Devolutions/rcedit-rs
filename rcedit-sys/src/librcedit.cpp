// C-API for rcedit library
#include <cstdint>
#include "rescle.h"

typedef struct ResourceUpdater ResourceUpdater;

extern "C"
{
    ResourceUpdater* ResourceUpdater_New()
    {
        return reinterpret_cast<ResourceUpdater*>(new rescle::ResourceUpdater());
    }

    void ResourceUpdater_Free(ResourceUpdater* ctx)
    {
        auto resourceUpdater = reinterpret_cast<rescle::ResourceUpdater*>(ctx);
        delete resourceUpdater;
    }

    bool ResourceUpdater_Load(ResourceUpdater* ctx, const wchar_t* wFilePath)
    {
        auto resourceUpdater = reinterpret_cast<rescle::ResourceUpdater*>(ctx);
        return resourceUpdater->Load(wFilePath);
    }

    bool ResourceUpdater_Commit(ResourceUpdater* ctx)
    {
        auto resourceUpdater = reinterpret_cast<rescle::ResourceUpdater*>(ctx);
        return resourceUpdater->Commit();
    }

    bool ResourceUpdater_SetIcon(ResourceUpdater* ctx, const wchar_t* wIconPath)
    {
        auto resourceUpdater = reinterpret_cast<rescle::ResourceUpdater*>(ctx);
        return resourceUpdater->SetIcon(wIconPath);
    }

    bool ResourceUpdater_SetVersionString(ResourceUpdater* ctx, const wchar_t* wName, const wchar_t* wValue)
    {
        auto resourceUpdater = reinterpret_cast<rescle::ResourceUpdater*>(ctx);
        return resourceUpdater->SetVersionString(wName, wValue);
    }

    bool ResourceUpdater_SetProductVersion(ResourceUpdater* ctx, unsigned short v1, unsigned short v2, unsigned short v3, unsigned short v4)
    {
        auto resourceUpdater = reinterpret_cast<rescle::ResourceUpdater*>(ctx);
        return resourceUpdater->SetProductVersion(v1, v2, v3, v4);
    }

    bool ResourceUpdater_SetFileVersion(ResourceUpdater* ctx, unsigned short v1, unsigned short v2, unsigned short v3, unsigned short v4)
    {
        auto resourceUpdater = reinterpret_cast<rescle::ResourceUpdater*>(ctx);
        return resourceUpdater->SetFileVersion(v1, v2, v3, v4);
    }

    bool ResourceUpdater_SetExecutionLevel(ResourceUpdater* ctx, const wchar_t* wLevel)
    {
        auto resourceUpdater = reinterpret_cast<rescle::ResourceUpdater*>(ctx);
        return resourceUpdater->SetExecutionLevel(wLevel);
    }    

    bool ResourceUpdater_ChangeRcdata(ResourceUpdater* ctx, uint32_t id, const wchar_t* wRcdataPath)
    {
        auto resourceUpdater = reinterpret_cast<rescle::ResourceUpdater*>(ctx);
        return resourceUpdater->ChangeRcData(id, wRcdataPath);
    }

    bool ResourceUpdater_ChangeString(ResourceUpdater* ctx, uint32_t id, const wchar_t* wString)
    {
        auto resourceUpdater = reinterpret_cast<rescle::ResourceUpdater*>(ctx);
        return resourceUpdater->ChangeString(id, wString);
    }
}