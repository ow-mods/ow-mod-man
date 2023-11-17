import {
    ReactNode,
    forwardRef,
    useCallback,
    useEffect,
    useImperativeHandle,
    useState
} from "react";
import { Config, GuiConfig, Language, OWMLConfig, Theme } from "@types";
import { useGetTranslation } from "@hooks";
import { commands } from "@commands";
import { TranslationNameMap } from "@components/common/TranslationContext";
import { os } from "@tauri-apps/api";
import { Box, useTheme } from "@mui/material";
import SettingsFolder from "./SettingsFolder";
import SettingsSelect from "./SettingsSelect";
import SettingsText from "./SettingsText";
import SettingsCheck from "./SettingsCheck";
import SettingsHeader from "./SettingsHeader";
import { simpleOnError } from "../../../../errorHandling";

const LanguageArr = Object.values(Language);
const ThemeArr = Object.values(Theme);

interface SettingsFormProps {
    initialConfig: Config;
    initialOwmlConfig: OWMLConfig;
    initialGuiConfig: GuiConfig;
}

export interface SettingsFormHandle {
    save: () => void;
    reset: () => void;
}

export interface SettingsRowProps {
    label: string;
    id: string;
    children?: ReactNode;
    tooltip?: string;
}

let defaultShowLogServerOption = false;

// Moved to out here due to #98
// Should work 99% of the time but the state is there just in case
os.platform().then((p) => (defaultShowLogServerOption = p === "win32"));

const SettingsForm = forwardRef(function SettingsForm(props: SettingsFormProps, ref) {
    const [config, setConfig] = useState<Config>(props.initialConfig);
    const [owmlConfig, setOwmlConfig] = useState<OWMLConfig>(props.initialOwmlConfig);
    const [guiConfig, setGuiConfig] = useState<GuiConfig>(props.initialGuiConfig);
    const getTranslation = useGetTranslation();
    const theme = useTheme();

    const [showLogServerOption, setShowLogServerOption] = useState(defaultShowLogServerOption);

    useEffect(() => {
        os.platform().then((p) => setShowLogServerOption(p === "win32"));
    }, []);

    useImperativeHandle(
        ref,
        () =>
            ({
                save: () => {
                    const task = async () => {
                        await commands.saveConfig({ config });
                        await commands.saveGuiConfig({ guiConfig });
                        if (config.owmlPath !== props.initialConfig.owmlPath) {
                            await commands.refreshLocalDb();
                        } else {
                            await commands.saveOwmlConfig({ owmlConfig });
                        }
                        if (config.databaseUrl !== props.initialConfig.databaseUrl) {
                            await commands.refreshRemoteDb();
                        }
                    };
                    task().catch(simpleOnError);
                },
                reset: () => {
                    setConfig(props.initialConfig);
                    setGuiConfig(props.initialGuiConfig);
                    setOwmlConfig(props.initialOwmlConfig);
                }
            }) as SettingsFormHandle,
        [
            config,
            owmlConfig,
            guiConfig,
            props.initialConfig,
            props.initialGuiConfig,
            props.initialOwmlConfig
        ]
    );

    const handleConf = (id: string, newVal: string | boolean) => {
        setConfig({ ...config, [id]: newVal });
    };

    const handleOwml = (id: string, newVal: string | boolean) => {
        setOwmlConfig({ ...owmlConfig, [id]: newVal });
    };

    const handleGui = (id: string, newVal: string | boolean) => {
        setGuiConfig({ ...guiConfig, [id]: newVal });
    };

    const onReset = useCallback((i: number) => {
        commands.getDefaultConfigs().then((data) => {
            // eslint-disable-next-line @typescript-eslint/ban-ts-comment
            // @ts-ignore
            [setConfig, setGuiConfig, setOwmlConfig][i](data[i]);
        });
    }, []);

    return (
        <Box display="flex" flexDirection="column" gap={theme.spacing(2)}>
            <SettingsHeader text={getTranslation("GUI_SETTINGS")} onReset={() => onReset(1)} />
            <SettingsSelect
                onChange={handleGui}
                value={guiConfig.language}
                translate={false}
                label={getTranslation("LANGUAGE")}
                options={LanguageArr}
                id="language"
                nameMap={TranslationNameMap}
            />
            <SettingsSelect
                onChange={handleGui}
                value={guiConfig.theme}
                translate
                label={getTranslation("THEME")}
                options={ThemeArr}
                id="theme"
            />
            <SettingsCheck
                onChange={handleGui}
                value={guiConfig.rainbow}
                label={getTranslation("RAINBOW")}
                id="rainbow"
                tooltip={getTranslation("TOOLTIP_RAINBOW")}
            />
            <SettingsCheck
                onChange={handleGui}
                value={guiConfig.watchFs}
                label={getTranslation("WATCH_FS")}
                id="watchFs"
                tooltip={getTranslation("TOOLTIP_WATCH_FS")}
            />
            <SettingsCheck
                onChange={handleGui}
                value={guiConfig.hideInstalledInRemote}
                label={getTranslation("HIDE_INSTALLED_MODS_IN_REMOTE")}
                id="hideInstalledInRemote"
                tooltip={getTranslation("TOOLTIP_HIDE_INSTALLED_MODS_IN_REMOTE")}
            />
            <SettingsCheck
                onChange={handleGui}
                value={guiConfig.hideModThumbnails}
                label={getTranslation("HIDE_MOD_THUMBNAILS")}
                id="hideModThumbnails"
                tooltip={getTranslation("TOOLTIP_HIDE_MOD_THUMBNAILS")}
            />
            <SettingsCheck
                onChange={handleGui}
                value={guiConfig.noWarning}
                label={getTranslation("DISABLE_WARNING")}
                id="noWarning"
                tooltip={getTranslation("TOOLTIP_DISABLE_WARNING")}
            />
            <SettingsCheck
                onChange={handleGui}
                value={guiConfig.logMultiWindow}
                label={getTranslation("LOG_MULTI_WINDOW")}
                id="logMultiWindow"
                tooltip={getTranslation("TOOLTIP_LOG_MULTI_WINDOW")}
            />
            <SettingsCheck
                onChange={handleGui}
                value={guiConfig.autoEnableDeps}
                label={getTranslation("AUTO_ENABLE_DEPS")}
                id="autoEnableDeps"
                tooltip={getTranslation("TOOLTIP_AUTO_ENABLE_DEPS")}
            />
            {showLogServerOption && (
                <SettingsCheck
                    onChange={handleGui}
                    value={guiConfig.noLogServer}
                    label={getTranslation("LET_OWML_HANDLE_LOGS")}
                    id="noLogServer"
                    tooltip={getTranslation("TOOLTIP_LET_OWML_HANDLE_LOGS")}
                />
            )}
            <SettingsHeader text={getTranslation("OWML_SETTINGS")} onReset={() => onReset(2)} />
            <SettingsFolder
                onChange={handleOwml}
                value={owmlConfig.gamePath}
                label={getTranslation("GAME_PATH")}
                id="gamePath"
                tooltip={getTranslation("TOOLTIP_GAME_PATH")}
            />
            <SettingsCheck
                onChange={handleOwml}
                value={owmlConfig.forceExe}
                label={getTranslation("FORCE_EXE")}
                id="forceExe"
                tooltip={getTranslation("TOOLTIP_FORCE_EXE")}
            />
            <SettingsCheck
                onChange={handleOwml}
                value={owmlConfig.debugMode}
                label={getTranslation("DEBUG_MODE")}
                id="debugMode"
                tooltip={getTranslation("TOOLTIP_OWML_DEBUG_MODE")}
            />
            <SettingsCheck
                onChange={handleOwml}
                value={owmlConfig.incrementalGC}
                label={getTranslation("INCREMENTAL_GC")}
                id="incrementalGC"
                tooltip={getTranslation("TOOLTIP_INCREMENTAL_GC")}
            />
            <SettingsHeader text={getTranslation("GENERAL_SETTINGS")} onReset={() => onReset(3)} />
            <SettingsText
                onChange={handleConf}
                value={config.databaseUrl}
                label={getTranslation("DB_URL")}
                id="databaseUrl"
                tooltip={getTranslation("TOOLTIP_DATABASE_URL")}
            />
            <SettingsText
                onChange={handleConf}
                value={config.alertUrl}
                label={getTranslation("ALERT_URL")}
                id="alertUrl"
                tooltip={getTranslation("TOOLTIP_ALERT_URL")}
            />
            <SettingsFolder
                onChange={handleConf}
                value={config.owmlPath}
                label={getTranslation("OWML_PATH")}
                id="owmlPath"
                tooltip={getTranslation("TOOLTIP_OWML_PATH")}
            />
        </Box>
    );
});

export default SettingsForm;
