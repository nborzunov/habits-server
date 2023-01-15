import { Box, Heading } from '@chakra-ui/react';
import Icons from '~/services/Icons';
import Statistics from '~/components/Habits/Statistics';
import { TargetCalendarContext, YearlyCalendar } from '~/components/Dashboard/YearlyCalendar';
import { useRecoilState, useRecoilValue, useSetRecoilState } from 'recoil';
import TargetChart from '~/components/Habits/TargetChart';
import { habitsState, layoutState, selectedHabitState } from '~/store/atoms';
import { Habit, TargetType } from '~/types/types';
import { useMutation } from '@tanstack/react-query';
import api from '~/services/api';
import MonthlyCalendar from '~/components/Dashboard/MonthlyCalendar';
import GridLayout, { Layout } from 'react-grid-layout';
import { useCallback, useEffect } from 'react';

export enum WidgetIdentifiers {
    CURRENT_STREAK = 'CURRENT_STREAK',
    COMPLETED_CHART = 'COMPLETED_CHART',
    COMPLETED_TARGETS = 'COMPLETED_TARGETS',
    FAILED_TARGETS = 'FAILED_TARGETS',
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
        lg: { x: 0, y: 0.9, w: 1, h: 1 },
    },
    FAILED_TARGETS: {
        lg: {
            x: 1,
            y: 1,
            w: 1,
            h: 1,
        },
    },

    YEARLY_CALENDAR: {
        lg: { x: 0, y: 2, w: 2, h: 2 },
    },
    MONTHLY_CALENDAR: {
        lg: {
            x: 2,
            y: 4,
            w: 1,
            h: 4,
        },
    },
    COMPLETED_CHART: {
        lg: {
            x: 2,
            y: 0,
            w: 1,
            h: 4,
        },
    },
};

const HabitDetails = () => {
    const habit = useRecoilValue(selectedHabitState);
    const setHabits = useSetRecoilState(habitsState);
    //
    const [layout, setLayout] = useRecoilState(layoutState);
    // const [widgets, setWidgets] = useLocalStorage<CustomWidget[]>(
    //     StorageKeys.WIDGETS,
    //     DEFAULT_WIDGET_STATE,
    // );

    useEffect(() => {
        if (layout) return;
        boostrapLayout();
    }, []);

    const boostrapLayout = useCallback(() => {
        const initial = Object.entries(WIDGET_LAYOUTS).map(([key, values]) => ({
            i: key,
            ...values[Object.keys(values).shift() as LayoutSizes],
        }));
        setLayout(initial as Layout[]);
    }, []);

    const onLayoutChange = useCallback(
        (newLayout: Layout[]) => {
            setLayout(newLayout.map((item) => Object.assign({}, item)));
        },
        [layout],
    );

    const createTarget = useMutation({
        mutationFn: (data: {
            id: string | null;
            habitId: string;
            date: Date;
            targetType: string;
        }) => {
            return api
                .post<Habit>('/targets/', data)
                .then((res) => res.data)
                .then((newHabit) =>
                    setHabits((prev) => prev.map((h) => (h.id !== newHabit.id ? h : newHabit))),
                );
        },
    });

    if (!habit) return null;

    const handleCalendarCellClick = (targetId: string | null, date: Date, newType: TargetType) => {
        createTarget.mutate({
            id: targetId,
            habitId: habit.id,
            date,
            targetType: newType,
        });
    };

    return (
        <Box m={0} width={'1600px'}>
            <Heading as='h3' px={2} size='md' p={2}>
                {habit.title}
            </Heading>
            <Box>
                <GridLayout
                    className='layout'
                    layout={layout}
                    cols={layoutWidth}
                    margin={[16, 16]}
                    rowHeight={layoutHeight}
                    width={1600}
                    isDraggable={true}
                    isResizable={false}
                    onLayoutChange={onLayoutChange}
                >
                    <Box key={WidgetIdentifiers.CURRENT_STREAK}>
                        <Statistics
                            title='Current streak'
                            value={habit.currentStreak}
                            type='streak'
                            startDate={habit.currentStreakStartDate}
                        />
                    </Box>

                    <Box
                        key={WidgetIdentifiers.COMPLETED_CHART}
                        borderRadius='xl'
                        borderColor='gray.200'
                        borderWidth='2px'
                        p='4'
                        display='flex'
                        justifyContent='center'
                    >
                        <TargetChart
                            completed={habit.completedTargets}
                            failed={habit.failedTargets}
                        />
                    </Box>

                    <Box key={WidgetIdentifiers.COMPLETED_TARGETS}>
                        <Statistics
                            icon={Icons.Complete}
                            title='Complete'
                            value={habit.completedTargets}
                            type='increase'
                            footerValue={habit.completedTargets}
                        />
                    </Box>
                    <Box key={WidgetIdentifiers.FAILED_TARGETS}>
                        <Statistics
                            icon={Icons.Cross}
                            title='Failed'
                            value={habit.failedTargets}
                            type='decrease'
                            footerValue={habit.failedTargets}
                        />
                    </Box>
                    <Box
                        key={WidgetIdentifiers.YEARLY_CALENDAR}
                        borderRadius='xl'
                        borderColor='gray.200'
                        borderWidth='2px'
                        p='2'
                        display='flex'
                        justifyContent='center'
                    >
                        <TargetCalendarContext.Provider
                            value={{
                                habit,
                                onCellClick: handleCalendarCellClick,
                            }}
                        >
                            <YearlyCalendar targets={habit.targets} />
                        </TargetCalendarContext.Provider>
                    </Box>
                    <Box
                        key={WidgetIdentifiers.MONTHLY_CALENDAR}
                        borderRadius='xl'
                        borderColor='gray.200'
                        borderWidth='2px'
                        p='2'
                        display='flex'
                        justifyContent='center'
                    >
                        <MonthlyCalendar
                            targets={habit.targets}
                            habit={habit}
                            onCellClick={handleCalendarCellClick}
                        />
                    </Box>
                </GridLayout>
            </Box>
        </Box>
    );
};

export default HabitDetails;
