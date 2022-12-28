#[doc = "Register `MRWCSR` reader"]
pub struct R(crate::R<MRWCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MRWCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MRWCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MRWCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MRWCSR` writer"]
pub struct W(crate::W<MRWCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MRWCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MRWCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MRWCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MRAL0` reader - Device 0 read address length setting"]
pub type MRAL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MRAL0` writer - Device 0 read address length setting"]
pub type MRAL0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MRWCSR_SPEC, u8, u8, 3, O>;
#[doc = "Field `MRCL0` reader - Device 0 read command length setting"]
pub type MRCL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MRCL0` writer - Device 0 read command length setting"]
pub type MRCL0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MRWCSR_SPEC, u8, u8, 3, O>;
#[doc = "Field `MRO0` reader - Device 0 read order setting"]
pub type MRO0_R = crate::BitReader<MRO0_A>;
#[doc = "Device 0 read order setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MRO0_A {
    #[doc = "0: Read order is byte0, byte1, byte2, byte3."]
    _0 = 0,
    #[doc = "1: Read order is byte1, byte0, byte3, byte2."]
    _1 = 1,
}
impl From<MRO0_A> for bool {
    #[inline(always)]
    fn from(variant: MRO0_A) -> Self {
        variant as u8 != 0
    }
}
impl MRO0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRO0_A {
        match self.bits {
            false => MRO0_A::_0,
            true => MRO0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MRO0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MRO0_A::_1
    }
}
#[doc = "Field `MRO0` writer - Device 0 read order setting"]
pub type MRO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MRWCSR_SPEC, MRO0_A, O>;
impl<'a, const O: u8> MRO0_W<'a, O> {
    #[doc = "Read order is byte0, byte1, byte2, byte3."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MRO0_A::_0)
    }
    #[doc = "Read order is byte1, byte0, byte3, byte2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MRO0_A::_1)
    }
}
#[doc = "Field `PREN0` reader - Preamble bit enable for mem0 memory-map read"]
pub type PREN0_R = crate::BitReader<PREN0_A>;
#[doc = "Preamble bit enable for mem0 memory-map read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PREN0_A {
    #[doc = "0: No check preamble bit"]
    _0 = 0,
    #[doc = "1: Check preamble bit from OctaFlash (if OctaFlash is connected to device 0)"]
    _1 = 1,
}
impl From<PREN0_A> for bool {
    #[inline(always)]
    fn from(variant: PREN0_A) -> Self {
        variant as u8 != 0
    }
}
impl PREN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREN0_A {
        match self.bits {
            false => PREN0_A::_0,
            true => PREN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PREN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PREN0_A::_1
    }
}
#[doc = "Field `PREN0` writer - Preamble bit enable for mem0 memory-map read"]
pub type PREN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MRWCSR_SPEC, PREN0_A, O>;
impl<'a, const O: u8> PREN0_W<'a, O> {
    #[doc = "No check preamble bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PREN0_A::_0)
    }
    #[doc = "Check preamble bit from OctaFlash (if OctaFlash is connected to device 0)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PREN0_A::_1)
    }
}
#[doc = "Field `MWAL0` reader - Device 0 write address length setting"]
pub type MWAL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MWAL0` writer - Device 0 write address length setting"]
pub type MWAL0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MRWCSR_SPEC, u8, u8, 3, O>;
#[doc = "Field `MWCL0` reader - Device 0 write command length setting"]
pub type MWCL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MWCL0` writer - Device 0 write command length setting"]
pub type MWCL0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MRWCSR_SPEC, u8, u8, 3, O>;
#[doc = "Field `MWO0` reader - Device 0 write order setting"]
pub type MWO0_R = crate::BitReader<MWO0_A>;
#[doc = "Device 0 write order setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MWO0_A {
    #[doc = "0: Write order is byte0, byte1, byte2, byte3."]
    _0 = 0,
    #[doc = "1: Write order is byte1, byte0, byte3, byte2."]
    _1 = 1,
}
impl From<MWO0_A> for bool {
    #[inline(always)]
    fn from(variant: MWO0_A) -> Self {
        variant as u8 != 0
    }
}
impl MWO0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWO0_A {
        match self.bits {
            false => MWO0_A::_0,
            true => MWO0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MWO0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MWO0_A::_1
    }
}
#[doc = "Field `MWO0` writer - Device 0 write order setting"]
pub type MWO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MRWCSR_SPEC, MWO0_A, O>;
impl<'a, const O: u8> MWO0_W<'a, O> {
    #[doc = "Write order is byte0, byte1, byte2, byte3."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MWO0_A::_0)
    }
    #[doc = "Write order is byte1, byte0, byte3, byte2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MWO0_A::_1)
    }
}
#[doc = "Field `MRAL1` reader - Device 1 read address length setting"]
pub type MRAL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MRAL1` writer - Device 1 read address length setting"]
pub type MRAL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MRWCSR_SPEC, u8, u8, 3, O>;
#[doc = "Field `MRCL1` reader - Device 1 read command length setting"]
pub type MRCL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MRCL1` writer - Device 1 read command length setting"]
pub type MRCL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MRWCSR_SPEC, u8, u8, 3, O>;
#[doc = "Field `MRO1` reader - Device 1 read order setting"]
pub type MRO1_R = crate::BitReader<MRO1_A>;
#[doc = "Device 1 read order setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MRO1_A {
    #[doc = "0: Read order is byte0, byte1, byte2, byte3."]
    _0 = 0,
    #[doc = "1: Read order is byte1, byte0, byte3, byte2."]
    _1 = 1,
}
impl From<MRO1_A> for bool {
    #[inline(always)]
    fn from(variant: MRO1_A) -> Self {
        variant as u8 != 0
    }
}
impl MRO1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRO1_A {
        match self.bits {
            false => MRO1_A::_0,
            true => MRO1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MRO1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MRO1_A::_1
    }
}
#[doc = "Field `MRO1` writer - Device 1 read order setting"]
pub type MRO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MRWCSR_SPEC, MRO1_A, O>;
impl<'a, const O: u8> MRO1_W<'a, O> {
    #[doc = "Read order is byte0, byte1, byte2, byte3."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MRO1_A::_0)
    }
    #[doc = "Read order is byte1, byte0, byte3, byte2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MRO1_A::_1)
    }
}
#[doc = "Field `PREN1` reader - Preamble bit enable for mem1 memory-map read"]
pub type PREN1_R = crate::BitReader<PREN1_A>;
#[doc = "Preamble bit enable for mem1 memory-map read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PREN1_A {
    #[doc = "0: No check preamble bit"]
    _0 = 0,
    #[doc = "1: Check preamble bit from OctaFlash (if OctaFlash is connected to device 1)"]
    _1 = 1,
}
impl From<PREN1_A> for bool {
    #[inline(always)]
    fn from(variant: PREN1_A) -> Self {
        variant as u8 != 0
    }
}
impl PREN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PREN1_A {
        match self.bits {
            false => PREN1_A::_0,
            true => PREN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PREN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PREN1_A::_1
    }
}
#[doc = "Field `PREN1` writer - Preamble bit enable for mem1 memory-map read"]
pub type PREN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MRWCSR_SPEC, PREN1_A, O>;
impl<'a, const O: u8> PREN1_W<'a, O> {
    #[doc = "No check preamble bit"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PREN1_A::_0)
    }
    #[doc = "Check preamble bit from OctaFlash (if OctaFlash is connected to device 1)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PREN1_A::_1)
    }
}
#[doc = "Field `MWAL1` reader - Device 1 write address length setting"]
pub type MWAL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MWAL1` writer - Device 1 write address length setting"]
pub type MWAL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MRWCSR_SPEC, u8, u8, 3, O>;
#[doc = "Field `MWCL1` reader - Device 1 write command length setting"]
pub type MWCL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MWCL1` writer - Device 1 write command length setting"]
pub type MWCL1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MRWCSR_SPEC, u8, u8, 3, O>;
#[doc = "Field `MWO1` reader - Device 1 write order setting"]
pub type MWO1_R = crate::BitReader<MWO1_A>;
#[doc = "Device 1 write order setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MWO1_A {
    #[doc = "0: Write order is byte0, byte1, byte2, byte3."]
    _0 = 0,
    #[doc = "1: Write order is byte1, byte0, byte3, byte2."]
    _1 = 1,
}
impl From<MWO1_A> for bool {
    #[inline(always)]
    fn from(variant: MWO1_A) -> Self {
        variant as u8 != 0
    }
}
impl MWO1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MWO1_A {
        match self.bits {
            false => MWO1_A::_0,
            true => MWO1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MWO1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MWO1_A::_1
    }
}
#[doc = "Field `MWO1` writer - Device 1 write order setting"]
pub type MWO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MRWCSR_SPEC, MWO1_A, O>;
impl<'a, const O: u8> MWO1_W<'a, O> {
    #[doc = "Write order is byte0, byte1, byte2, byte3."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MWO1_A::_0)
    }
    #[doc = "Write order is byte1, byte0, byte3, byte2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MWO1_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Device 0 read address length setting"]
    #[inline(always)]
    pub fn mral0(&self) -> MRAL0_R {
        MRAL0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Device 0 read command length setting"]
    #[inline(always)]
    pub fn mrcl0(&self) -> MRCL0_R {
        MRCL0_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - Device 0 read order setting"]
    #[inline(always)]
    pub fn mro0(&self) -> MRO0_R {
        MRO0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Preamble bit enable for mem0 memory-map read"]
    #[inline(always)]
    pub fn pren0(&self) -> PREN0_R {
        PREN0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Device 0 write address length setting"]
    #[inline(always)]
    pub fn mwal0(&self) -> MWAL0_R {
        MWAL0_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - Device 0 write command length setting"]
    #[inline(always)]
    pub fn mwcl0(&self) -> MWCL0_R {
        MWCL0_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - Device 0 write order setting"]
    #[inline(always)]
    pub fn mwo0(&self) -> MWO0_R {
        MWO0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Device 1 read address length setting"]
    #[inline(always)]
    pub fn mral1(&self) -> MRAL1_R {
        MRAL1_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - Device 1 read command length setting"]
    #[inline(always)]
    pub fn mrcl1(&self) -> MRCL1_R {
        MRCL1_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bit 22 - Device 1 read order setting"]
    #[inline(always)]
    pub fn mro1(&self) -> MRO1_R {
        MRO1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Preamble bit enable for mem1 memory-map read"]
    #[inline(always)]
    pub fn pren1(&self) -> PREN1_R {
        PREN1_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Device 1 write address length setting"]
    #[inline(always)]
    pub fn mwal1(&self) -> MWAL1_R {
        MWAL1_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Device 1 write command length setting"]
    #[inline(always)]
    pub fn mwcl1(&self) -> MWCL1_R {
        MWCL1_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bit 30 - Device 1 write order setting"]
    #[inline(always)]
    pub fn mwo1(&self) -> MWO1_R {
        MWO1_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Device 0 read address length setting"]
    #[inline(always)]
    #[must_use]
    pub fn mral0(&mut self) -> MRAL0_W<0> {
        MRAL0_W::new(self)
    }
    #[doc = "Bits 3:5 - Device 0 read command length setting"]
    #[inline(always)]
    #[must_use]
    pub fn mrcl0(&mut self) -> MRCL0_W<3> {
        MRCL0_W::new(self)
    }
    #[doc = "Bit 6 - Device 0 read order setting"]
    #[inline(always)]
    #[must_use]
    pub fn mro0(&mut self) -> MRO0_W<6> {
        MRO0_W::new(self)
    }
    #[doc = "Bit 7 - Preamble bit enable for mem0 memory-map read"]
    #[inline(always)]
    #[must_use]
    pub fn pren0(&mut self) -> PREN0_W<7> {
        PREN0_W::new(self)
    }
    #[doc = "Bits 8:10 - Device 0 write address length setting"]
    #[inline(always)]
    #[must_use]
    pub fn mwal0(&mut self) -> MWAL0_W<8> {
        MWAL0_W::new(self)
    }
    #[doc = "Bits 11:13 - Device 0 write command length setting"]
    #[inline(always)]
    #[must_use]
    pub fn mwcl0(&mut self) -> MWCL0_W<11> {
        MWCL0_W::new(self)
    }
    #[doc = "Bit 14 - Device 0 write order setting"]
    #[inline(always)]
    #[must_use]
    pub fn mwo0(&mut self) -> MWO0_W<14> {
        MWO0_W::new(self)
    }
    #[doc = "Bits 16:18 - Device 1 read address length setting"]
    #[inline(always)]
    #[must_use]
    pub fn mral1(&mut self) -> MRAL1_W<16> {
        MRAL1_W::new(self)
    }
    #[doc = "Bits 19:21 - Device 1 read command length setting"]
    #[inline(always)]
    #[must_use]
    pub fn mrcl1(&mut self) -> MRCL1_W<19> {
        MRCL1_W::new(self)
    }
    #[doc = "Bit 22 - Device 1 read order setting"]
    #[inline(always)]
    #[must_use]
    pub fn mro1(&mut self) -> MRO1_W<22> {
        MRO1_W::new(self)
    }
    #[doc = "Bit 23 - Preamble bit enable for mem1 memory-map read"]
    #[inline(always)]
    #[must_use]
    pub fn pren1(&mut self) -> PREN1_W<23> {
        PREN1_W::new(self)
    }
    #[doc = "Bits 24:26 - Device 1 write address length setting"]
    #[inline(always)]
    #[must_use]
    pub fn mwal1(&mut self) -> MWAL1_W<24> {
        MWAL1_W::new(self)
    }
    #[doc = "Bits 27:29 - Device 1 write command length setting"]
    #[inline(always)]
    #[must_use]
    pub fn mwcl1(&mut self) -> MWCL1_W<27> {
        MWCL1_W::new(self)
    }
    #[doc = "Bit 30 - Device 1 write order setting"]
    #[inline(always)]
    #[must_use]
    pub fn mwo1(&mut self) -> MWO1_W<30> {
        MWO1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory Map Read/Write Setting Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mrwcsr](index.html) module"]
pub struct MRWCSR_SPEC;
impl crate::RegisterSpec for MRWCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mrwcsr::R](R) reader structure"]
impl crate::Readable for MRWCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mrwcsr::W](W) writer structure"]
impl crate::Writable for MRWCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MRWCSR to value 0"]
impl crate::Resettable for MRWCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
