import { hooks } from "@commands";
import { useGetTranslation } from "@hooks";
import { ComputerRounded, PublicRounded, UpdateRounded } from "@mui/icons-material";
import TabList from "@mui/lab/TabList";
import { AppBar, useTheme } from "@mui/material";
import Tab from "@mui/material/Tab";
import { FunctionComponent } from "react";

const AppTabs: FunctionComponent<{ onChange: (newVal: string) => void }> = ({ onChange }) => {
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
                    value="1"
                    icon={<ComputerRounded />}
                    iconPosition="start"
                    label={getTranslation("INSTALLED_MODS")}
                />
                <Tab
                    value="2"
                    icon={<PublicRounded />}
                    iconPosition="start"
                    label={getTranslation("GET_MODS")}
                />
                <Tab
                    value="3"
                    icon={<UpdateRounded />}
                    iconPosition="start"
                    label={getTranslation("UPDATES", { amount: countText })}
                />
            </TabList>
        </AppBar>
    );
};

export default AppTabs;
