import { hooks } from "@commands";
import { useGetTranslation } from "@hooks";
import { ComputerRounded, PublicRounded, UpdateRounded } from "@mui/icons-material";
import TabList from "@mui/lab/TabList";
import { AppBar, useTheme } from "@mui/material";
import Tab from "@mui/material/Tab";
import { memo } from "react";

export type ModsTab = "local" | "remote" | "updates";

export interface AppTabsProps {
    onChange: (newVal: ModsTab) => void;
}

const AppTabs = memo(function AppTabs({ onChange }: AppTabsProps) {
    const getTranslation = useGetTranslation();
    const theme = useTheme();
    const count =
        hooks.getUpdatableMods(["LOCAL-REFRESH", "REMOTE-REFRESH"], { filter: "" })[1]?.length ?? 0;
    const countText = count === 0 ? "" : `(${count})`;

    return (
        <AppBar position="static">
            <TabList
                sx={{ margin: `0 ${theme.spacing(3)}` }}
                onChange={(_, newVal) => onChange(newVal)}
                variant="fullWidth"
                textColor="inherit"
                indicatorColor="secondary"
            >
                <Tab
                    value="local"
                    icon={<ComputerRounded />}
                    iconPosition="start"
                    label={getTranslation("INSTALLED_MODS")}
                />
                <Tab
                    value="remote"
                    icon={<PublicRounded />}
                    iconPosition="start"
                    label={getTranslation("GET_MODS")}
                />
                <Tab
                    value="updates"
                    icon={<UpdateRounded />}
                    iconPosition="start"
                    label={getTranslation("UPDATES", { amount: countText })}
                />
            </TabList>
        </AppBar>
    );
});

export default AppTabs;
