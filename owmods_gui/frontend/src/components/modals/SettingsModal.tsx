import {
    ChangeEvent,
    ReactNode,
    forwardRef,
    memo,
    useCallback,
    useImperativeHandle,
    useRef,
    useState
} from "react";
import { Config, GuiConfig, Language, OWMLConfig, Theme } from "@types";
import Modal from "./Modal";
import { useTranslation, useTranslations } from "@hooks";
import { commands, hooks } from "@commands";
import { OpenFileInput } from "@components/common/FileInput";
import Icon from "@components/common/Icon";
import { BsArrowCounterclockwise } from "react-icons/bs";
import { TranslationNameMap } from "@components/common/TranslationContext";

const ThemeArr = Object.values(Theme);
const LanguageArr = Object.values(Language);

interface SettingsFormProps {
    initialConfig: Config;
    initialOwmlConfig: OWMLConfig;
    initialGuiConfig: GuiConfig;
}

interface SettingsFormHandle {
    save: () => void;
}

interface SettingsRowProps {
    label: string;
    id: string;
    children?: ReactNode;
    tooltip?: string;
    tooltipPlacement?: string;
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
    nameMap?: Record<string, string>;
}

const SettingsRow = (props: SettingsRowProps) => {
    return (
        <div className="settings-row">
            <label
                data-tooltip={props.tooltip}
                data-placement={props.tooltipPlacement ?? "bottom"}
                htmlFor={props.id}
            >
                {props.label}
            </label>
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
    let translations = useTranslations(Array.from(props.options));

    if (!props.translate) {
        translations = Array.from(props.options);
    }

    return (
        <SettingsRow {...props}>
            <select value={props.value} id={props.id} onChange={(e) => props.onChange?.(e)}>
                {props.options.map((o, i) => (
                    <option key={o} value={o}>
                        {props.translate ? translations[i] : props.nameMap?.[o] ?? o}
                    </option>
                ))}
            </select>
        </SettingsRow>
    );
};

const SettingsFolder = (props: SettingsTextProps) => {
    const title = useTranslation("SELECT", { name: props.label });

    const onChange = (e: string) => {
        props.onChange?.({
            target: {
                id: props.id,
                value: e
            }
        } as ChangeEvent<HTMLInputElement>);
    };

    return (
        <OpenFileInput
            id={props.id}
            label={props.label}
            value={props.value}
            onChange={onChange}
            className="settings-row"
            dialogOptions={{
                defaultPath: props.value,
                directory: true,
                multiple: false,
                title
            }}
            tooltip={props.tooltip}
            tooltipPlacement={props.tooltipPlacement}
        />
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
            <label data-tooltip={props.tooltip} data-placement="right" htmlFor={props.id}>
                {props.label}
            </label>
        </div>
    );
};

const ResetButton = memo(function ResetButton(props: { onClick: () => void }) {
    const resetTooltip = useTranslation("RESET");

    return (
        <a
            className="reset-button"
            aria-label={resetTooltip}
            data-placement="left"
            data-tooltip={resetTooltip}
            onClick={props.onClick}
            href="#"
        >
            <Icon iconType={BsArrowCounterclockwise} />
        </a>
    );
});

const SettingsForm = forwardRef(function SettingsForm(props: SettingsFormProps, ref) {
    const [config, setConfig] = useState<Config>(props.initialConfig);
    const [owmlConfig, setOwmlConfig] = useState<OWMLConfig>(props.initialOwmlConfig);
    const [guiConfig, setGuiConfig] = useState<GuiConfig>(props.initialGuiConfig);

    useImperativeHandle(
        ref,
        () => ({
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
                task().catch(console.error);
            }
        }),
        [config, owmlConfig, guiConfig, props.initialConfig]
    );

    const [
        generalSettings,
        dbUrl,
        alertUrl,
        owmlPath,
        theme,
        rainbow,
        language,
        watchFs,
        disableWarning,
        logMultiWindow,
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
        "THEME",
        "RAINBOW",
        "LANGUAGE",
        "WATCH_FS",
        "DISABLE_WARNING",
        "LOG_MULTI_WINDOW",
        "GAME_PATH",
        "FORCE_EXE",
        "DEBUG_MODE",
        "INCREMENTAL_GC",
        "OWML_SETTINGS",
        "GUI_SETTINGS"
    ]);

    const [
        dbUrlTooltip,
        alertUrlTooltip,
        owmlPathTooltip,
        incrementalGCTooltip,
        owmlDebugModeTooltip,
        directExeTooltip,
        gamePathTooltip,
        logMultiTooltip,
        disableWarningTooltip,
        watchFSTooltip
    ] = useTranslations([
        "TOOLTIP_DATABASE_URL",
        "TOOLTIP_ALERT_URL",
        "TOOLTIP_OWML_PATH",
        "TOOLTIP_INCREMENTAL_GC",
        "TOOLTIP_OWML_DEBUG_MODE",
        "TOOLTIP_FORCE_EXE",
        "TOOLTIP_GAME_PATH",
        "TOOLTIP_LOG_MULTI_WINDOW",
        "TOOLTIP_DISABLE_WARNING",
        "TOOLTIP_WATCH_FS"
    ]);

    const getVal = (e: HTMLInputElement | HTMLSelectElement) => {
        const type = e.type;
        if (type && type === "checkbox") {
            return (e as HTMLInputElement).checked;
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

    const onReset = useCallback((i: number) => {
        commands.getDefaultConfigs().then((data) => {
            // eslint-disable-next-line @typescript-eslint/ban-ts-comment
            // @ts-ignore
            [setConfig, setGuiConfig, setOwmlConfig][i](data[i]);
        });
    }, []);

    return (
        <form className="settings">
            <h4>
                {guiSettingsLabel} <ResetButton onClick={() => onReset(1)} />
            </h4>
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
                nameMap={TranslationNameMap}
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
                tooltip={watchFSTooltip}
            />
            <SettingsSwitch
                onChange={handleGui}
                value={guiConfig.noWarning}
                label={disableWarning}
                id="noWarning"
                tooltip={disableWarningTooltip}
            />
            <SettingsSwitch
                onChange={handleGui}
                value={guiConfig.logMultiWindow}
                label={logMultiWindow}
                id="logMultiWindow"
                tooltip={logMultiTooltip}
            />
            <h4>
                {owmlSettingsLabel} <ResetButton onClick={() => onReset(2)} />
            </h4>
            <SettingsFolder
                onChange={handleOwml}
                value={owmlConfig.gamePath}
                label={gamePath}
                id="gamePath"
                tooltip={gamePathTooltip}
            />
            <SettingsSwitch
                onChange={handleOwml}
                value={owmlConfig.forceExe}
                label={forceExe}
                id="forceExe"
                tooltip={directExeTooltip}
            />
            <SettingsSwitch
                onChange={handleOwml}
                value={owmlConfig.debugMode}
                label={debugMode}
                id="debugMode"
                tooltip={owmlDebugModeTooltip}
            />
            <SettingsSwitch
                onChange={handleOwml}
                value={owmlConfig.incrementalGC}
                label={incrementalGC}
                id="incrementalGC"
                tooltip={incrementalGCTooltip}
            />
            <h4>
                {generalSettings} <ResetButton onClick={() => onReset(0)} />
            </h4>
            <SettingsText
                onChange={handleConf}
                value={config.databaseUrl}
                label={dbUrl}
                id="databaseUrl"
                tooltip={dbUrlTooltip}
            />
            <SettingsText
                onChange={handleConf}
                value={config.alertUrl}
                label={alertUrl}
                id="alertUrl"
                tooltip={alertUrlTooltip}
            />
            <SettingsFolder
                onChange={handleConf}
                value={config.owmlPath}
                label={owmlPath}
                id="owmlPath"
                tooltip={owmlPathTooltip}
                tooltipPlacement="top"
            />
        </form>
    );
});

const SettingsModal = forwardRef(function SettingsModal(_: object, ref) {
    const settingsFormRef = useRef<SettingsFormHandle>();

    const [configStatus, config, err1] = hooks.getConfig("CONFIG_RELOAD");
    const [guiConfigStatus, guiConfig, err2] = hooks.getGuiConfig("GUI_CONFIG_RELOAD");
    const [owmlConfigStatus, owmlConfig, err3] = hooks.getOwmlConfig("OWML_CONFIG_RELOAD");

    const status = [configStatus, guiConfigStatus, owmlConfigStatus];

    const [settings, save] = useTranslations(["SETTINGS", "SAVE"]);

    if (status.includes("Loading")) {
        return <></>;
    } else if (status.includes("Error")) {
        return (
            <Modal showCancel heading={settings} confirmText={save} ref={ref}>
                <>
                    <p className="center">
                        <>
                            Error: Couldn&apos;t Load Settings: {err1 ?? ""} {err2 ?? ""}{" "}
                            {err3 ?? ""}
                        </>
                    </p>
                </>
            </Modal>
        );
    } else {
        return (
            <Modal
                onConfirm={() => settingsFormRef.current?.save()}
                showCancel
                heading={settings}
                confirmText={save}
                ref={ref}
            >
                <SettingsForm
                    ref={settingsFormRef}
                    initialConfig={config!}
                    initialGuiConfig={guiConfig!}
                    initialOwmlConfig={owmlConfig!}
                />
            </Modal>
        );
    }
});

export default SettingsModal;
