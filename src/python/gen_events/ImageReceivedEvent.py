# automatically generated by the FlatBuffers compiler, do not modify

# namespace: gen_events

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class ImageReceivedEvent(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = ImageReceivedEvent()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsImageReceivedEvent(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    # ImageReceivedEvent
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # ImageReceivedEvent
    def EventCreateTs(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # ImageReceivedEvent
    def ImageUuid(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

def ImageReceivedEventStart(builder): builder.StartObject(2)
def Start(builder):
    return ImageReceivedEventStart(builder)
def ImageReceivedEventAddEventCreateTs(builder, eventCreateTs): builder.PrependUOffsetTRelativeSlot(0, flatbuffers.number_types.UOffsetTFlags.py_type(eventCreateTs), 0)
def AddEventCreateTs(builder, eventCreateTs):
    return ImageReceivedEventAddEventCreateTs(builder, eventCreateTs)
def ImageReceivedEventAddImageUuid(builder, imageUuid): builder.PrependUOffsetTRelativeSlot(1, flatbuffers.number_types.UOffsetTFlags.py_type(imageUuid), 0)
def AddImageUuid(builder, imageUuid):
    return ImageReceivedEventAddImageUuid(builder, imageUuid)
def ImageReceivedEventEnd(builder): return builder.EndObject()
def End(builder):
    return ImageReceivedEventEnd(builder)