import { Box, Flex, Grid, GridItem, Text, Tooltip, useTheme } from '@chakra-ui/react';
import dayjs from 'dayjs';
import getLoop from '~/common/utils/getLoop';
import React, { useContext } from 'react';
import { Habit, Target, TargetType } from '~/Habits/types';

interface TargetCalendarContext {
    habit?: Habit;
    onCellClick?: (targetId: string | null, date: Date, newType: TargetType) => void;
}

export const TargetCalendarContext = React.createContext<TargetCalendarContext>({});

export const YearlyCalendar = ({ size, targets }: { size?: 'sm' | 'md'; targets: Target[] }) => {
    const targetsMap = targets.reduce((acc, target) => {
        acc[dayjs(target.date).format('DD/MM/YYYY')] = target;
        return acc;
    }, {} as { [key: string]: Target });
    size = size || 'md';
    return (
        <Box height={'100%'}>
            <Flex p='2'>
                {getLoop(12).map((i) => (
                    <Month
                        key={i}
                        monthId={i}
                        size={size === 'sm' ? 0 : 1}
                        targetsMap={targetsMap}
                    />
                ))}
            </Flex>
        </Box>
    );
};

const Month = ({
    monthId,
    size,
    targetsMap,
}: {
    monthId: number;
    size: number;
    targetsMap: Record<string, Target>;
}) => {
    const daysInMonth = dayjs(`2023-${monthId + 1}-1`).daysInMonth();
    const firstDay = dayjs(`2023-${monthId + 1}-1`).day();
    const columns = Math.ceil((firstDay + daysInMonth) / 7);

    const gaps = [3, 3];
    const cellSizes = [12, 12];
    const gap = gaps[size];
    const cellSize = cellSizes[size];

    const month = dayjs(`2023-${monthId + 1}-1`).format('MMM');
    return (
        <Box p='1'>
            <Text pb='1' textAlign='center' fontWeight='bold'>
                {month}
            </Text>
            <Grid templateColumns={`repeat(${columns}, ${cellSize}px)`} gap={`${gap}px`}>
                {getLoop(columns).map((columnId) => (
                    <GridItem key={monthId + columnId}>
                        <Grid templateRows={`repeat(7, ${cellSize}px)`} gap={`${gap}px`}>
                            {getLoop(7).map((rowId) => (
                                <Cell
                                    key={monthId + columnId + rowId}
                                    monthId={monthId}
                                    dayId={columnId * 7 + rowId - firstDay}
                                    daysInMonth={daysInMonth}
                                    size={cellSize}
                                    targetsMap={targetsMap}
                                />
                            ))}
                        </Grid>
                    </GridItem>
                ))}
            </Grid>
        </Box>
    );
};

const Cell = ({
    dayId,
    monthId,
    daysInMonth,
    size,
    targetsMap,
}: {
    dayId: number;
    monthId: number;
    daysInMonth: number;
    size: number;
    targetsMap: Record<string, Target>;
}) => {
    const { habit, onCellClick } = useContext(TargetCalendarContext);
    const sizePx = `${size}px`;
    const theme = useTheme();

    if (dayId < 0 || dayId >= daysInMonth) {
        return <Box key={'empty' + monthId + dayId} width={sizePx} height={sizePx} />;
    }

    const green = theme.colors.green;

    const day = dayjs(`2023-${monthId + 1}-${dayId + 1}`);

    const target = targetsMap[day.format('DD/MM/YYYY')];

    const handleClick = () => {
        let newType;

        if (!target) {
            newType = TargetType.Done;
        } else if (target.targetType === TargetType.Done && habit?.allowSkip) {
            newType = TargetType.Skip;
        } else {
            newType = TargetType.Empty;
        }

        onCellClick?.(target?.id, day.toDate(), newType);
    };
    return (
        <Box key={monthId + dayId} cursor='pointer'>
            <Tooltip label={dayjs(`2023-${monthId + 1}-${dayId + 1}`).format('D MMMM YYYY')}>
                {target && target.targetType === TargetType.Skip ? (
                    <Box
                        width={sizePx}
                        height={sizePx}
                        bg={'gray.300'}
                        borderTop='10px solid transparent'
                        borderWidth={`${sizePx} 0 0 ${sizePx}`}
                        borderColor={`transparent transparent transparent ${green[500]}}`}
                        onClick={handleClick}
                    ></Box>
                ) : (
                    <Box
                        width={sizePx}
                        height={sizePx}
                        bg={target ? 'green.500' : 'gray.300'}
                        onClick={handleClick}
                    ></Box>
                )}
            </Tooltip>
        </Box>
    );
};
