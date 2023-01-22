import { useEffect, useState } from 'react';
import { Layout } from 'react-grid-layout';
import { useRecoilState } from 'recoil';
import { layoutState } from '~/common/store/atoms';

export enum WidgetIdentifiers {
    CURRENT_STREAK = 'CURRENT_STREAK',
    COMPLETED_CHART = 'COMPLETED_CHART',
    COMPLETED_TARGETS = 'COMPLETED_TARGETS',
    FAILED_TARGETS = 'FAILED_TARGETS',
    TOTAL_TARGETS = 'TOTAL_TARGETS',
    SKIPPED_TARGETS = 'SKIPPED_TARGETS',
    YEARLY_CALENDAR = 'YEARLY_CALENDAR',
    MONTHLY_CALENDAR = 'MONTHLY_CALENDAR',
}

type LayoutPropsOnly = Omit<Layout, 'i'>;
export type LayoutSizes = 'sm' | 'lg';
const layoutWidth = 3;
const layoutHeight = 94;

const WIDGET_LAYOUTS: Record<WidgetIdentifiers, Partial<Record<LayoutSizes, LayoutPropsOnly>>> = {
    CURRENT_STREAK: {
        lg: { x: 0, y: 0, w: 2, h: 1 },
    },
    COMPLETED_TARGETS: {
        lg: { x: 0, y: 1, w: 1, h: 1 },
    },
    FAILED_TARGETS: {
        lg: {
            x: 1,
            y: 1,
            w: 1,
            h: 1,
        },
    },
    TOTAL_TARGETS: {
        lg: { x: 0, y: 2, w: 1, h: 1 },
    },
    SKIPPED_TARGETS: {
        lg: { x: 1, y: 2, w: 1, h: 1 },
    },
    YEARLY_CALENDAR: {
        lg: { x: 0, y: 2, w: 2, h: 2, isResizable: false },
    },
    COMPLETED_CHART: {
        lg: {
            x: 2,
            y: 0,
            w: 1,
            h: 4,
            isResizable: false,
        },
    },
    MONTHLY_CALENDAR: {
        lg: {
            x: 2,
            y: 4,
            w: 1,
            h: 3.5,
            isResizable: false,
        },
    },
};

const useWidgets = (isEditMode: boolean) => {
    const [layout, setLayout] = useRecoilState(layoutState);
    const [newLayout, setNewLayout] = useState<Layout[]>([]);

    useEffect(() => {
        if (layout) return;
        setLayout(boostrapLayout());
    }, []);

    const boostrapLayout = () => {
        const initial = Object.entries(WIDGET_LAYOUTS).map(([key, values]) => ({
            i: key,
            ...values[Object.keys(values).shift() as LayoutSizes],
            resizeHandles: ['e', 'w'],
        })) as Layout[];

        setNewLayout(initial);

        return initial;
    };

    const onLayoutChange = (newLayout: Layout[]) => {
        setNewLayout(newLayout.map((item) => item));
    };

    const removeWidget = (id: WidgetIdentifiers) => {
        setNewLayout(currentWidgetLayout.filter((item) => item.i !== id));
    };

    const save = () => {
        setLayout(newLayout);
    };

    const reset = () => {
        boostrapLayout();
    };

    const currentWidgetLayout = (isEditMode ? newLayout : layout) as Layout[];

    const widgets = Object.values(WidgetIdentifiers).filter(
        (key) => !currentWidgetLayout || !currentWidgetLayout.find((item) => item.i === key),
    );

    return {
        save,
        reset,
        removeWidget,
        widgets,
        props: {
            className: 'layout',
            layout: isEditMode ? newLayout : layout,
            cols: layoutWidth,
            margin: [16, 16] as [number, number],
            rowHeight: layoutHeight,
            width: 1600,
            isDraggable: isEditMode,
            isResizable: isEditMode,
            onLayoutChange: onLayoutChange,
        },
    };
};

export default useWidgets;
