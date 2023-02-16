import Icon from "@components/Icon";
import { ChangeEvent, MutableRefObject, ReactNode, useEffect, useRef, useState } from "react";
import { FaFolder } from "react-icons/fa";
import { Config, GuiConfig, LanguageArr, OwmlConfig, ThemeArr } from "@types";
import Modal, { ModalWrapperProps } from "./Modal";
import { useTauri, useTranslations } from "@hooks";
import { dialog, invoke, os } from "@tauri-apps/api";

interface SettingsFormProps {
    initialConfig: Config;
    initialOwmlConfig: OwmlConfig;
    initialGuiConfig: GuiConfig;
    save: MutableRefObject<() => void>;
}

interface SettingsRowProps {
    label: string;
    id: string;
    children?: ReactNode;
}

interface SettingsTextProps extends SettingsRowProps {
    value: string;
    onChange?: (e: ChangeEvent<HTMLInputElement>) => void;
}

interface SettingsSwitchProps extends SettingsRowProps {
    value: boolean;
    onChange?: (e: ChangeEvent<HTMLInputElement>) => void;
}

interface SettingsSelectProps extends SettingsRowProps {
    value: string;
    options: readonly string[];
    translate: boolean;
    onChange?: (e: ChangeEvent<HTMLSelectElement>) => void;
}

const SettingsRow = (props: SettingsRowProps) => {
    return (
        <div className="settings-row">
            <label htmlFor={props.id}>{props.label}</label>
            <div>{props.children}</div>
        </div>
    );
};

const SettingsText = (props: SettingsTextProps) => {
    return (
        <SettingsRow {...props}>
            <input
                onChange={(e) => props.onChange?.(e)}
                name={props.id}
                value={props.value}
                id={props.id}
            />
            {props.children}
        </SettingsRow>
    );
};

const SettingsSelect = (props: SettingsSelectProps) => {
    let translations = Array.from(props.options);

    if (props.translate) {
        translations = useTranslations(Array.from(props.options));
    }

    return (
        <SettingsRow {...props}>
            <select value={props.value} id={props.id} onChange={(e) => props.onChange?.(e)}>
                {props.options.map((o, i) => (
                    <option key={o} value={o}>
                        {translations[i]}
                    </option>
                ))}
            </select>
        </SettingsRow>
    );
};

const SettingsFolder = (props: SettingsTextProps) => {
    const [browse, select] = useTranslations(["BROWSE", "SELECT"]);

    const onBrowse = () => {
        dialog
            .open({
                defaultPath: props.value,
                directory: true,
                multiple: false,
                title: `${select} ${props.label}`
            })
            .then((path) => {
                if (path !== null) {
                    props.onChange?.({
                        target: { id: props.id, value: path }
                    } as ChangeEvent<HTMLInputElement>);
                }
            });
    };

    return (
        <SettingsText {...props}>
            <button onClick={onBrowse} className="fix-icons" type="button">
                <Icon iconType={FaFolder} /> {browse}
            </button>
        </SettingsText>
    );
};

const SettingsSwitch = (props: SettingsSwitchProps) => {
    return (
        <div className="settings-row fix-icons">
            <input
                onChange={(e) => props.onChange?.(e)}
                type="checkbox"
                role="switch"
                checked={props.value}
                id={props.id}
                name={props.id}
            />
            <label htmlFor={props.id}>{props.label}</label>
        </div>
    );
};

const SettingsForm = (props: SettingsFormProps) => {
    const [config, setConfig] = useState<Config>(props.initialConfig);
    const [owmlConfig, setOwmlConfig] = useState<OwmlConfig>(props.initialOwmlConfig);
    const [guiConfig, setGuiConfig] = useState<GuiConfig>(props.initialGuiConfig);
    const [platform, setPlatform] = useState("windows");

    const [
        generalSettings,
        dbUrl,
        alertUrl,
        owmlPath,
        winePrefix,
        theme,
        rainbow,
        language,
        watchFs,
        gamePath,
        forceExe,
        debugMode,
        incrementalGC,
        owmlSettingsLabel,
        guiSettingsLabel
    ] = useTranslations([
        "GENERAL_SETTINGS",
        "DB_URL",
        "ALERT_URL",
        "OWML_PATH",
        "WINE_PREFIX",
        "THEME",
        "RAINBOW",
        "LANGUAGE",
        "WATCH_FS",
        "GAME_PATH",
        "FORCE_EXE",
        "DEBUG_MODE",
        "INCREMENTAL_GC",
        "OWML_SETTINGS",
        "GUI_SETTINGS"
    ]);

    const getVal = (e: HTMLInputElement | HTMLSelectElement) => {
        const type = (e as any).type;
        if (type && type === "checkbox") {
            return (e as any).checked;
        } else {
            return e.value;
        }
    };

    const handleConf = (e: ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
        setConfig({ ...config, [e.target.id]: getVal(e.target) });
    };

    const handleOwml = (e: ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
        setOwmlConfig({ ...owmlConfig, [e.target.id]: getVal(e.target) });
    };

    const handleGui = (e: ChangeEvent<HTMLInputElement | HTMLSelectElement>) => {
        setGuiConfig({ ...guiConfig, [e.target.id]: getVal(e.target) });
    };

    useEffect(() => {
        os.platform().then(setPlatform);
    }, []);

    props.save.current = () => {
        const task = async () => {
            await invoke("save_config", { config });
            await invoke("save_gui_config", { guiConfig });
            if (config.owmlPath !== props.initialConfig.owmlPath) {
                await invoke("refresh_local_db");
            } else {
                await invoke("save_owml_config", { owmlConfig });
            }
            if (config.databaseUrl !== props.initialConfig.databaseUrl) {
                await invoke("refresh_remote_db");
            }
        };
        task().catch(console.error);
    };

    return (
        <form className="settings">
            <h4>{generalSettings}</h4>
            <SettingsText
                onChange={handleConf}
                value={config.databaseUrl}
                label={dbUrl}
                id="databaseUrl"
            />
            <SettingsText
                onChange={handleConf}
                value={config.alertUrl}
                label={alertUrl}
                id="alertUrl"
            />
            <SettingsFolder
                onChange={handleConf}
                value={config.owmlPath}
                label={owmlPath}
                id="owmlPath"
            />
            {platform === "linux" && (
                <SettingsFolder
                    onChange={handleConf}
                    value={config.winePrefix}
                    label={winePrefix}
                    id="winePrefix"
                />
            )}
            <h4>{guiSettingsLabel}</h4>
            <SettingsSelect
                onChange={handleGui}
                value={guiConfig.theme}
                translate
                label={theme}
                options={ThemeArr}
                id="theme"
            />
            <SettingsSelect
                onChange={handleGui}
                value={guiConfig.language}
                translate={false}
                label={language}
                options={LanguageArr}
                id="language"
            />
            <SettingsSwitch
                onChange={handleGui}
                value={guiConfig.rainbow}
                label={rainbow}
                id="rainbow"
            />
            <SettingsSwitch
                onChange={handleGui}
                value={guiConfig.watchFs}
                label={watchFs}
                id="watchFs"
            />
            <h4>{owmlSettingsLabel}</h4>
            <SettingsFolder
                onChange={handleOwml}
                value={owmlConfig.gamePath}
                label={gamePath}
                id="gamePath"
            />
            <SettingsSwitch
                onChange={handleOwml}
                value={owmlConfig.forceExe}
                label={forceExe}
                id="forceExe"
            />
            <SettingsSwitch
                onChange={handleOwml}
                value={owmlConfig.debugMode}
                label={debugMode}
                id="debugMode"
            />
            <SettingsSwitch
                onChange={handleOwml}
                value={owmlConfig.incrementalGC}
                label={incrementalGC}
                id="incrementalGC"
            />
        </form>
    );
};

const SettingsModal = (props: ModalWrapperProps) => {
    const [configStatus, config, err1] = useTauri<Config>("CONFIG_RELOAD", "fetch_config");
    const [guiConfigStatus, guiConfig, err2] = useTauri<GuiConfig>(
        "GUI_CONFIG_RELOAD",
        "get_gui_config"
    );
    const [owmlConfigStatus, owmlConfig, err3] = useTauri<OwmlConfig>(
        "OWML_CONFIG_RELOAD",
        "get_owml_config"
    );

    const saveChanges = useRef<() => void>(() => null);

    const status = [configStatus, guiConfigStatus, owmlConfigStatus];

    const [settings, save] = useTranslations(["SETTINGS", "SAVE"]);

    if (status.includes("Loading")) {
        return <></>;
    } else if (status.includes("Error")) {
        return (
            <Modal showCancel heading={settings} confirmText={save} open={props.open}>
                <>
                    <p className="center-loading">
                        Error: Couldn&apos;t Load Settings: {err1 ?? ""} {err2 ?? ""} {err3 ?? ""}
                    </p>
                </>
            </Modal>
        );
    } else {
        return (
            <Modal
                onConfirm={() => saveChanges.current()}
                showCancel
                heading={settings}
                confirmText={save}
                open={props.open}
            >
                <SettingsForm
                    save={saveChanges}
                    initialConfig={config!}
                    initialGuiConfig={guiConfig!}
                    initialOwmlConfig={owmlConfig!}
                />
            </Modal>
        );
    }
};

export default SettingsModal;
