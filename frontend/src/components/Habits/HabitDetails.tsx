import { Box, Button, Flex, Heading, HStack, Icon, IconButton, Tooltip } from '@chakra-ui/react';
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
import { useEffect, useState } from 'react';

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

const HabitDetails = () => {
    const habit = useRecoilValue(selectedHabitState);
    const setHabits = useSetRecoilState(habitsState);
    const [layout, setLayout] = useRecoilState(layoutState);
    const [newLayout, setNewLayout] = useState<Layout[]>([]);
    const [isEditMode, setIsEditMode] = useState(false);

    useEffect(() => {
        if (layout) return;
        boostrapLayout();
    }, []);

    const boostrapLayout = () => {
        const initial = Object.entries(WIDGET_LAYOUTS).map(([key, values]) => ({
            i: key,
            ...values[Object.keys(values).shift() as LayoutSizes],
            resizeHandles: ['e', 'w'],
        }));
        setLayout(initial as Layout[]);
        setNewLayout(initial as Layout[]);
    };

    const onLayoutChange = (newLayout: Layout[]) => {
        setNewLayout(newLayout.map((item) => Object.assign({}, item)));
    };

    // TODO: вынести мутейшены в отдельный файл
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

    const save = () => {
        setLayout(newLayout);
        setIsEditMode(false);
    };

    const reset = () => {
        boostrapLayout();
    };
    return (
        <Box m={0} width={'1600px'}>
            <Flex alignItems={'center'} justifyContent={'space-between'} px={4} pt={2}>
                <Heading as='h3' size='md'>
                    {habit.title}
                </Heading>
                <HStack spacing={2}>
                    {isEditMode && (
                        <HStack spacing={2}>
                            <Tooltip label={'Save'}>
                                <IconButton
                                    aria-label={'save widgets'}
                                    icon={<Icon as={Icons.Save} />}
                                    onClick={save}
                                    colorScheme={'purple'}
                                />
                            </Tooltip>

                            <Tooltip label={'Reset'}>
                                <Button colorScheme={'purple'} variant={'outline'} onClick={reset}>
                                    Reset
                                </Button>
                            </Tooltip>

                            <Tooltip label={'Close'}>
                                <IconButton
                                    aria-label={'close'}
                                    icon={<Icon as={Icons.Cross} />}
                                    onClick={() => setIsEditMode(!isEditMode)}
                                    colorScheme={'purple'}
                                    variant={'outline'}
                                />
                            </Tooltip>
                        </HStack>
                    )}

                    {!isEditMode && (
                        <Tooltip label={'Edit grid'}>
                            <IconButton
                                aria-label={'edit grid'}
                                icon={<Icon as={Icons.Grid} />}
                                onClick={() => setIsEditMode(!isEditMode)}
                                colorScheme={'purple'}
                                variant={'outline'}
                            />
                        </Tooltip>
                    )}
                </HStack>
            </Flex>
            <Box userSelect={isEditMode ? 'none' : 'auto'}>
                <GridLayout
                    className='layout'
                    layout={isEditMode ? newLayout : layout}
                    cols={layoutWidth}
                    margin={[16, 16]}
                    rowHeight={layoutHeight}
                    width={1600}
                    isDraggable={isEditMode}
                    isResizable={isEditMode}
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
