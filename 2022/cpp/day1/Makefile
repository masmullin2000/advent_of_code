CPP=g++ -static
OUT=out

OPTS=-flto -Ofast -I../utils/src/

vpath %.cpp src/
vpath %.cpp ../utils/src/

UTILS=utils.cpp lib.cpp

D1_FILES=day1p1.cpp $(UTILS)
D1_OBJS=$(patsubst %.cpp,$(OUT)/%.o,$(D1_FILES))

D2_FILES=day1p2.cpp $(UTILS)
D2_OBJS=$(patsubst %.cpp,$(OUT)/%.o,$(D2_FILES))

build: day1p1 day1p2

day1p1: $(OUT) $(D1_OBJS)
	#$(CPP) -o $@ $(D1_OBJS) -L/usr/lib/ -lboost_iostreams
	$(CPP) -o $@ $(D1_OBJS)

day1p2: $(OUT) $(D2_OBJS)
	#$(CPP) -o $@ $(D2_OBJS) -L/usr/lib/ -lboost_iostreams
	$(CPP) -o $@ $(D2_OBJS)

clean:
	rm -rf $(OUT)

$(OUT):
	-mkdir $(OUT)
	@echo $(D1_FILES)
	@echo $(D1_SRCS)
	@echo $(D1_OBJS)

$(OUT)/%.o : %.cpp
	$(CPP) $(OPTS) -c $< -o $@
