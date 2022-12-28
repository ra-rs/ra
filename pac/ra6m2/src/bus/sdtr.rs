#[doc = "Register `SDTR` reader"]
pub struct R(crate::R<SDTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SDTR` writer"]
pub struct W(crate::W<SDTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDTR_SPEC>;
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
impl From<crate::W<SDTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CL` reader - SDRAMC Column Latency"]
pub type CL_R = crate::FieldReader<u8, CL_A>;
#[doc = "SDRAMC Column Latency\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CL_A {
    #[doc = "1: 1 cycle"]
    _001 = 1,
    #[doc = "2: 2 cycles"]
    _010 = 2,
    #[doc = "3: 3 cycles"]
    _011 = 3,
}
impl From<CL_A> for u8 {
    #[inline(always)]
    fn from(variant: CL_A) -> Self {
        variant as _
    }
}
impl CL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CL_A> {
        match self.bits {
            1 => Some(CL_A::_001),
            2 => Some(CL_A::_010),
            3 => Some(CL_A::_011),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CL_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CL_A::_011
    }
}
#[doc = "Field `CL` writer - SDRAMC Column Latency"]
pub type CL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDTR_SPEC, u8, CL_A, 3, O>;
impl<'a, const O: u8> CL_W<'a, O> {
    #[doc = "1 cycle"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(CL_A::_001)
    }
    #[doc = "2 cycles"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CL_A::_010)
    }
    #[doc = "3 cycles"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CL_A::_011)
    }
}
#[doc = "Field `WR` reader - Write Recovery Interval"]
pub type WR_R = crate::BitReader<WR_A>;
#[doc = "Write Recovery Interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WR_A {
    #[doc = "0: 1 cycle"]
    _0 = 0,
    #[doc = "1: 2 cycles"]
    _1 = 1,
}
impl From<WR_A> for bool {
    #[inline(always)]
    fn from(variant: WR_A) -> Self {
        variant as u8 != 0
    }
}
impl WR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WR_A {
        match self.bits {
            false => WR_A::_0,
            true => WR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WR_A::_1
    }
}
#[doc = "Field `WR` writer - Write Recovery Interval"]
pub type WR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SDTR_SPEC, WR_A, O>;
impl<'a, const O: u8> WR_W<'a, O> {
    #[doc = "1 cycle"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WR_A::_0)
    }
    #[doc = "2 cycles"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WR_A::_1)
    }
}
#[doc = "Field `RP` reader - Row Precharge Interval ( RP+1 cycles )"]
pub type RP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RP` writer - Row Precharge Interval ( RP+1 cycles )"]
pub type RP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDTR_SPEC, u8, u8, 3, O>;
#[doc = "Field `RCD` reader - Row Column Latency ( RCD+1 cycles )"]
pub type RCD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCD` writer - Row Column Latency ( RCD+1 cycles )"]
pub type RCD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SDTR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RAS` reader - Row Active Interval"]
pub type RAS_R = crate::FieldReader<u8, RAS_A>;
#[doc = "Row Active Interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RAS_A {
    #[doc = "0: 1 cycle"]
    _000 = 0,
    #[doc = "1: 2 cycles"]
    _001 = 1,
    #[doc = "2: 3 cycles"]
    _010 = 2,
    #[doc = "3: 4 cycles"]
    _011 = 3,
    #[doc = "4: 5 cycles"]
    _100 = 4,
    #[doc = "5: 6 cycles"]
    _101 = 5,
    #[doc = "6: 7 cycles"]
    _110 = 6,
    #[doc = "7: Setting prohibited"]
    _111 = 7,
}
impl From<RAS_A> for u8 {
    #[inline(always)]
    fn from(variant: RAS_A) -> Self {
        variant as _
    }
}
impl RAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAS_A {
        match self.bits {
            0 => RAS_A::_000,
            1 => RAS_A::_001,
            2 => RAS_A::_010,
            3 => RAS_A::_011,
            4 => RAS_A::_100,
            5 => RAS_A::_101,
            6 => RAS_A::_110,
            7 => RAS_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == RAS_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == RAS_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == RAS_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == RAS_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == RAS_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == RAS_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == RAS_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == RAS_A::_111
    }
}
#[doc = "Field `RAS` writer - Row Active Interval"]
pub type RAS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SDTR_SPEC, u8, RAS_A, 3, O>;
impl<'a, const O: u8> RAS_W<'a, O> {
    #[doc = "1 cycle"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(RAS_A::_000)
    }
    #[doc = "2 cycles"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(RAS_A::_001)
    }
    #[doc = "3 cycles"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(RAS_A::_010)
    }
    #[doc = "4 cycles"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(RAS_A::_011)
    }
    #[doc = "5 cycles"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(RAS_A::_100)
    }
    #[doc = "6 cycles"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(RAS_A::_101)
    }
    #[doc = "7 cycles"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(RAS_A::_110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(RAS_A::_111)
    }
}
impl R {
    #[doc = "Bits 0:2 - SDRAMC Column Latency"]
    #[inline(always)]
    pub fn cl(&self) -> CL_R {
        CL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - Write Recovery Interval"]
    #[inline(always)]
    pub fn wr(&self) -> WR_R {
        WR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Row Precharge Interval ( RP+1 cycles )"]
    #[inline(always)]
    pub fn rp(&self) -> RP_R {
        RP_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Row Column Latency ( RCD+1 cycles )"]
    #[inline(always)]
    pub fn rcd(&self) -> RCD_R {
        RCD_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:18 - Row Active Interval"]
    #[inline(always)]
    pub fn ras(&self) -> RAS_R {
        RAS_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SDRAMC Column Latency"]
    #[inline(always)]
    #[must_use]
    pub fn cl(&mut self) -> CL_W<0> {
        CL_W::new(self)
    }
    #[doc = "Bit 8 - Write Recovery Interval"]
    #[inline(always)]
    #[must_use]
    pub fn wr(&mut self) -> WR_W<8> {
        WR_W::new(self)
    }
    #[doc = "Bits 9:11 - Row Precharge Interval ( RP+1 cycles )"]
    #[inline(always)]
    #[must_use]
    pub fn rp(&mut self) -> RP_W<9> {
        RP_W::new(self)
    }
    #[doc = "Bits 12:13 - Row Column Latency ( RCD+1 cycles )"]
    #[inline(always)]
    #[must_use]
    pub fn rcd(&mut self) -> RCD_W<12> {
        RCD_W::new(self)
    }
    #[doc = "Bits 16:18 - Row Active Interval"]
    #[inline(always)]
    #[must_use]
    pub fn ras(&mut self) -> RAS_W<16> {
        RAS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDRAM Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdtr](index.html) module"]
pub struct SDTR_SPEC;
impl crate::RegisterSpec for SDTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdtr::R](R) reader structure"]
impl crate::Readable for SDTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sdtr::W](W) writer structure"]
impl crate::Writable for SDTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDTR to value 0x02"]
impl crate::Resettable for SDTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
