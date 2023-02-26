import { useTranslations } from "@hooks";
import { SocketMessageType } from "@types";
import { memo } from "react";
import { LogFilter } from "./App";

export interface LogHeaderProps {
    activeFilter: LogFilter;
    setActiveFilter: (filter: LogFilter) => void;
    autoScroll: boolean;
    setAutoScroll: (newValue: boolean) => void;
}

const LogHeader = memo(
    (props: LogHeaderProps) => {
        const [filterLabel, autoScrollLabel, anyLabel] = useTranslations([
            "FILTER",
            "AUTO_SCROLL",
            "ANY"
        ]);

        const filterTranslations = useTranslations(Object.keys(SocketMessageType));

        return (
            <>
                <div className="log-actions">
                    <label htmlFor="filter">
                        {filterLabel}
                        <select
                            id="filter"
                            value={props.activeFilter}
                            onChange={(e) => props.setActiveFilter(e.target.value as LogFilter)}
                        >
                            <>
                                <option value="Any">{anyLabel}</option>
                                {Object.keys(SocketMessageType).map((k, i) => {
                                    {
                                        return (
                                            k !== "Fatal" &&
                                            k !== "Quit" && (
                                                <option key={k} value={k}>
                                                    {filterTranslations[i]}
                                                </option>
                                            )
                                        );
                                    }
                                })}
                            </>
                        </select>
                    </label>
                    <label htmlFor="scroll">
                        {autoScrollLabel}
                        <input
                            id="scroll"
                            type="checkbox"
                            role="switch"
                            checked={props.autoScroll}
                            onChange={(e) => props.setAutoScroll(e.target.checked)}
                        />
                    </label>
                </div>
            </>
        );
    },
    (current, next) =>
        current.activeFilter === next.activeFilter && current.autoScroll === next.autoScroll
);

export default LogHeader;
