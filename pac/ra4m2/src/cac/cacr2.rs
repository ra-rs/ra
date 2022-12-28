#[doc = "Register `CACR2` reader"]
pub struct R(crate::R<CACR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACR2` writer"]
pub struct W(crate::W<CACR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACR2_SPEC>;
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
impl From<crate::W<CACR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPS` reader - Reference Signal Select"]
pub type RPS_R = crate::BitReader<RPS_A>;
#[doc = "Reference Signal Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RPS_A {
    #[doc = "0: CACREF pin input"]
    _0 = 0,
    #[doc = "1: Internal clock (internally generated signal)"]
    _1 = 1,
}
impl From<RPS_A> for bool {
    #[inline(always)]
    fn from(variant: RPS_A) -> Self {
        variant as u8 != 0
    }
}
impl RPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPS_A {
        match self.bits {
            false => RPS_A::_0,
            true => RPS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RPS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RPS_A::_1
    }
}
#[doc = "Field `RPS` writer - Reference Signal Select"]
pub type RPS_W<'a, const O: u8> = crate::BitWriter<'a, u8, CACR2_SPEC, RPS_A, O>;
impl<'a, const O: u8> RPS_W<'a, O> {
    #[doc = "CACREF pin input"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RPS_A::_0)
    }
    #[doc = "Internal clock (internally generated signal)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RPS_A::_1)
    }
}
#[doc = "Field `RSCS` reader - Measurement Reference Clock Select"]
pub type RSCS_R = crate::FieldReader<u8, RSCS_A>;
#[doc = "Measurement Reference Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSCS_A {
    #[doc = "0: Main clock oscillator"]
    _000 = 0,
    #[doc = "1: Sub-clock oscillator"]
    _001 = 1,
    #[doc = "2: HOCO clock"]
    _010 = 2,
    #[doc = "3: MOCO clock"]
    _011 = 3,
    #[doc = "4: LOCO clock"]
    _100 = 4,
    #[doc = "5: Peripheral module clock B (PCLKB)"]
    _101 = 5,
    #[doc = "6: IWDT-dedicated clock"]
    _110 = 6,
    #[doc = "7: Setting prohibited"]
    _111 = 7,
}
impl From<RSCS_A> for u8 {
    #[inline(always)]
    fn from(variant: RSCS_A) -> Self {
        variant as _
    }
}
impl RSCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSCS_A {
        match self.bits {
            0 => RSCS_A::_000,
            1 => RSCS_A::_001,
            2 => RSCS_A::_010,
            3 => RSCS_A::_011,
            4 => RSCS_A::_100,
            5 => RSCS_A::_101,
            6 => RSCS_A::_110,
            7 => RSCS_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == RSCS_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == RSCS_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == RSCS_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == RSCS_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == RSCS_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == RSCS_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == RSCS_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == RSCS_A::_111
    }
}
#[doc = "Field `RSCS` writer - Measurement Reference Clock Select"]
pub type RSCS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CACR2_SPEC, u8, RSCS_A, 3, O>;
impl<'a, const O: u8> RSCS_W<'a, O> {
    #[doc = "Main clock oscillator"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(RSCS_A::_000)
    }
    #[doc = "Sub-clock oscillator"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(RSCS_A::_001)
    }
    #[doc = "HOCO clock"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(RSCS_A::_010)
    }
    #[doc = "MOCO clock"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(RSCS_A::_011)
    }
    #[doc = "LOCO clock"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(RSCS_A::_100)
    }
    #[doc = "Peripheral module clock B (PCLKB)"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(RSCS_A::_101)
    }
    #[doc = "IWDT-dedicated clock"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(RSCS_A::_110)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(RSCS_A::_111)
    }
}
#[doc = "Field `RCDS` reader - Measurement Reference Clock Frequency Division Ratio Select"]
pub type RCDS_R = crate::FieldReader<u8, RCDS_A>;
#[doc = "Measurement Reference Clock Frequency Division Ratio Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RCDS_A {
    #[doc = "0: x 1/32 clock"]
    _00 = 0,
    #[doc = "1: x 1/128 clock"]
    _01 = 1,
    #[doc = "2: x 1/1024 clock"]
    _10 = 2,
    #[doc = "3: x 1/8192 clock"]
    _11 = 3,
}
impl From<RCDS_A> for u8 {
    #[inline(always)]
    fn from(variant: RCDS_A) -> Self {
        variant as _
    }
}
impl RCDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCDS_A {
        match self.bits {
            0 => RCDS_A::_00,
            1 => RCDS_A::_01,
            2 => RCDS_A::_10,
            3 => RCDS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RCDS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RCDS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RCDS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == RCDS_A::_11
    }
}
#[doc = "Field `RCDS` writer - Measurement Reference Clock Frequency Division Ratio Select"]
pub type RCDS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CACR2_SPEC, u8, RCDS_A, 2, O>;
impl<'a, const O: u8> RCDS_W<'a, O> {
    #[doc = "x 1/32 clock"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RCDS_A::_00)
    }
    #[doc = "x 1/128 clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RCDS_A::_01)
    }
    #[doc = "x 1/1024 clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RCDS_A::_10)
    }
    #[doc = "x 1/8192 clock"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(RCDS_A::_11)
    }
}
#[doc = "Field `DFS` reader - Digital Filter Select"]
pub type DFS_R = crate::FieldReader<u8, DFS_A>;
#[doc = "Digital Filter Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DFS_A {
    #[doc = "0: Disable digital filtering"]
    _00 = 0,
    #[doc = "1: Use sampling clock for the digital filter as the frequency measuring clock"]
    _01 = 1,
    #[doc = "2: Use sampling clock for the digital filter as the frequency measuring clock divided by 4"]
    _10 = 2,
    #[doc = "3: Use sampling clock for the digital filter as the frequency measuring clock divided by 16."]
    _11 = 3,
}
impl From<DFS_A> for u8 {
    #[inline(always)]
    fn from(variant: DFS_A) -> Self {
        variant as _
    }
}
impl DFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFS_A {
        match self.bits {
            0 => DFS_A::_00,
            1 => DFS_A::_01,
            2 => DFS_A::_10,
            3 => DFS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DFS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DFS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DFS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DFS_A::_11
    }
}
#[doc = "Field `DFS` writer - Digital Filter Select"]
pub type DFS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CACR2_SPEC, u8, DFS_A, 2, O>;
impl<'a, const O: u8> DFS_W<'a, O> {
    #[doc = "Disable digital filtering"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DFS_A::_00)
    }
    #[doc = "Use sampling clock for the digital filter as the frequency measuring clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DFS_A::_01)
    }
    #[doc = "Use sampling clock for the digital filter as the frequency measuring clock divided by 4"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DFS_A::_10)
    }
    #[doc = "Use sampling clock for the digital filter as the frequency measuring clock divided by 16."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DFS_A::_11)
    }
}
impl R {
    #[doc = "Bit 0 - Reference Signal Select"]
    #[inline(always)]
    pub fn rps(&self) -> RPS_R {
        RPS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Measurement Reference Clock Select"]
    #[inline(always)]
    pub fn rscs(&self) -> RSCS_R {
        RSCS_R::new((self.bits >> 1) & 7)
    }
    #[doc = "Bits 4:5 - Measurement Reference Clock Frequency Division Ratio Select"]
    #[inline(always)]
    pub fn rcds(&self) -> RCDS_R {
        RCDS_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Digital Filter Select"]
    #[inline(always)]
    pub fn dfs(&self) -> DFS_R {
        DFS_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Reference Signal Select"]
    #[inline(always)]
    #[must_use]
    pub fn rps(&mut self) -> RPS_W<0> {
        RPS_W::new(self)
    }
    #[doc = "Bits 1:3 - Measurement Reference Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn rscs(&mut self) -> RSCS_W<1> {
        RSCS_W::new(self)
    }
    #[doc = "Bits 4:5 - Measurement Reference Clock Frequency Division Ratio Select"]
    #[inline(always)]
    #[must_use]
    pub fn rcds(&mut self) -> RCDS_W<4> {
        RCDS_W::new(self)
    }
    #[doc = "Bits 6:7 - Digital Filter Select"]
    #[inline(always)]
    #[must_use]
    pub fn dfs(&mut self) -> DFS_W<6> {
        DFS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CAC Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cacr2](index.html) module"]
pub struct CACR2_SPEC;
impl crate::RegisterSpec for CACR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cacr2::R](R) reader structure"]
impl crate::Readable for CACR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cacr2::W](W) writer structure"]
impl crate::Writable for CACR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACR2 to value 0"]
impl crate::Resettable for CACR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
