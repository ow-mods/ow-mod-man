import Icon from "@components/Icon";
import { ChangeEvent, ReactNode, useState } from "react";
import { FaFolder } from "react-icons/fa";
import { Config, defaultConfig, defaultOwmlConfig, OwmlConfig } from "@types";
import Modal, { ModalWrapperProps } from "./Modal";
import { useTranslation, useTranslations } from "@hooks";

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

const SettingsFolder = (props: SettingsTextProps) => {
    const browse = useTranslation("BROWSE");

    return (
        <SettingsText {...props}>
            <button className="fix-icons" type="button">
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

const SettingsModal = (props: ModalWrapperProps) => {
    const [config, setConfig] = useState<Config>(defaultConfig);
    const [owmlConfig, setOwmlConfig] = useState<OwmlConfig>(defaultOwmlConfig);

    const handleConf = (e: ChangeEvent<HTMLInputElement>) => {
        setConfig({ ...config, [e.target.id]: e.target.value });
    };

    const handleOwml = (e: ChangeEvent<HTMLInputElement>) => {
        setOwmlConfig({ ...owmlConfig, [e.target.id]: e.target.value });
    };

    const [
        settings,
        save,
        generalSettings,
        dbUrl,
        alertUrl,
        owmlPath,
        winePrefix,
        gamePath,
        forceExe,
        debugMode,
        incrementalGC,
        owmlSettingsLabel
    ] = useTranslations([
        "SETTINGS",
        "SAVE",
        "GENERAL_SETTINGS",
        "DB_URL",
        "ALERT_URL",
        "OWML_PATH",
        "WINE_PREFIX",
        "GAME_PATH",
        "FORCE_EXE",
        "DEBUG_MODE",
        "INCREMENTAL_GC",
        "OWML_SETTINGS"
    ]);

    return (
        <Modal showCancel heading={settings} confirmText={save} open={props.open}>
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
                <SettingsFolder
                    onChange={handleConf}
                    value={config.winePrefix}
                    label={winePrefix}
                    id="winePrefix"
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
        </Modal>
    );
};

export default SettingsModal;
