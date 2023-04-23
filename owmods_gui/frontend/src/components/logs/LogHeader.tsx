import { useTranslation, useTranslations } from "@hooks";
import { SocketMessageType } from "@types";
import { memo, useCallback, useRef, useState } from "react";
import { LogFilter } from "./LogApp";

export interface LogHeaderProps {
    logsLen: number;
    activeFilter: LogFilter;
    setActiveFilter: (filter: LogFilter) => void;
    activeSearch: string;
    setActiveSearch: (newSearch: string) => void;
    onClear: () => void;
}

const LogHeader = memo(
    (props: LogHeaderProps) => {
        const [tempSearch, setTempSearch] = useState<string>("");
        const searchTimeout = useRef<number | undefined>(undefined);
        const [filterLabel, searchLogs, anyLabel, clearLabel] = useTranslations([
            "FILTER",
            "SEARCH_LOGS",
            "ANY",
            "CLEAR_LOGS"
        ]);

        const logCountLabel = useTranslation("LOG_COUNT", { count: props.logsLen.toString() });

        const onSearchChange = useCallback(
            (val: string) => {
                setTempSearch(val);
                if (searchTimeout) clearTimeout(searchTimeout.current);
                searchTimeout.current = setTimeout(() => {
                    props.setActiveSearch(val);
                }, 200);
            },
            [tempSearch]
        );

        const filterTranslations = useTranslations(Object.keys(SocketMessageType));

        return (
            <>
                <div className="log-actions">
                    <label htmlFor="search">
                        {searchLogs}
                        <input
                            type="text"
                            id="search"
                            value={tempSearch}
                            onChange={(e) => onSearchChange(e.target.value)}
                            placeholder="Search"
                        />
                    </label>
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
                    <div>
                        <span>{logCountLabel}</span>
                        <a
                            href={props.logsLen === 0 ? undefined : "#"}
                            role="button"
                            onClick={() => props.onClear()}
                        >
                            {clearLabel}
                        </a>
                    </div>
                </div>
            </>
        );
    },
    (current, next) =>
        current.activeFilter === next.activeFilter && current.logsLen === next.logsLen
);

export default LogHeader;
