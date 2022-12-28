#[doc = "Register `JIFECNT` reader"]
pub struct R(crate::R<JIFECNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JIFECNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JIFECNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JIFECNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JIFECNT` writer"]
pub struct W(crate::W<JIFECNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JIFECNT_SPEC>;
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
impl From<crate::W<JIFECNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JIFECNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DINSWAP` reader - Byte/Halfword Swap"]
pub type DINSWAP_R = crate::FieldReader<u8, DINSWAP_A>;
#[doc = "Byte/Halfword Swap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DINSWAP_A {
    #[doc = "0: (1) (2) (3) (4) (5) (6) (7) (8)"]
    _000 = 0,
    #[doc = "1: (2) (1) (4) (3) (6) (5) (8) (7) \\[Byte swap\\]"]
    _001 = 1,
    #[doc = "2: (3) (4) (1) (2) (7) (8) (5) (6) \\[Halfword swap\\]"]
    _010 = 2,
    #[doc = "3: (4) (3) (2) (1) (8) (7) (6) (5) \\[Halfword - byte swap\\]"]
    _011 = 3,
    #[doc = "4: (5) (6) (7) (8) (1) (2) (3) (4) \\[Word swap\\]"]
    _100 = 4,
    #[doc = "5: (6) (5) (8) (7) (2) (1) (4) (3) \\[Word - byte swap\\]"]
    _101 = 5,
    #[doc = "6: (7) (8) (5) (6) (3) (4) (1) (2) \\[Word - Halfword swap\\]"]
    _110 = 6,
    #[doc = "7: (8) (7) (6) (5) (4) (3) (2) (1) \\[Word - Halfword - byte swap\\]"]
    _111 = 7,
}
impl From<DINSWAP_A> for u8 {
    #[inline(always)]
    fn from(variant: DINSWAP_A) -> Self {
        variant as _
    }
}
impl DINSWAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINSWAP_A {
        match self.bits {
            0 => DINSWAP_A::_000,
            1 => DINSWAP_A::_001,
            2 => DINSWAP_A::_010,
            3 => DINSWAP_A::_011,
            4 => DINSWAP_A::_100,
            5 => DINSWAP_A::_101,
            6 => DINSWAP_A::_110,
            7 => DINSWAP_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == DINSWAP_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == DINSWAP_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == DINSWAP_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == DINSWAP_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == DINSWAP_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == DINSWAP_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == DINSWAP_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == DINSWAP_A::_111
    }
}
#[doc = "Field `DINSWAP` writer - Byte/Halfword Swap"]
pub type DINSWAP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, JIFECNT_SPEC, u8, DINSWAP_A, 3, O>;
impl<'a, const O: u8> DINSWAP_W<'a, O> {
    #[doc = "(1) (2) (3) (4) (5) (6) (7) (8)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(DINSWAP_A::_000)
    }
    #[doc = "(2) (1) (4) (3) (6) (5) (8) (7) \\[Byte swap\\]"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(DINSWAP_A::_001)
    }
    #[doc = "(3) (4) (1) (2) (7) (8) (5) (6) \\[Halfword swap\\]"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(DINSWAP_A::_010)
    }
    #[doc = "(4) (3) (2) (1) (8) (7) (6) (5) \\[Halfword - byte swap\\]"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(DINSWAP_A::_011)
    }
    #[doc = "(5) (6) (7) (8) (1) (2) (3) (4) \\[Word swap\\]"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(DINSWAP_A::_100)
    }
    #[doc = "(6) (5) (8) (7) (2) (1) (4) (3) \\[Word - byte swap\\]"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(DINSWAP_A::_101)
    }
    #[doc = "(7) (8) (5) (6) (3) (4) (1) (2) \\[Word - Halfword swap\\]"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(DINSWAP_A::_110)
    }
    #[doc = "(8) (7) (6) (5) (4) (3) (2) (1) \\[Word - Halfword - byte swap\\]"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(DINSWAP_A::_111)
    }
}
#[doc = "Field `DINLC` reader - Count Mode Setting for Stopping Input Image Data Lines"]
pub type DINLC_R = crate::BitReader<DINLC_A>;
#[doc = "Count Mode Setting for Stopping Input Image Data Lines\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DINLC_A {
    #[doc = "0: Count mode for stopping the input of image data lines is off"]
    _0 = 0,
    #[doc = "1: Count mode for stopping the input of image data lines is on"]
    _1 = 1,
}
impl From<DINLC_A> for bool {
    #[inline(always)]
    fn from(variant: DINLC_A) -> Self {
        variant as u8 != 0
    }
}
impl DINLC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINLC_A {
        match self.bits {
            false => DINLC_A::_0,
            true => DINLC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DINLC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DINLC_A::_1
    }
}
#[doc = "Field `DINLC` writer - Count Mode Setting for Stopping Input Image Data Lines"]
pub type DINLC_W<'a, const O: u8> = crate::BitWriter<'a, u32, JIFECNT_SPEC, DINLC_A, O>;
impl<'a, const O: u8> DINLC_W<'a, O> {
    #[doc = "Count mode for stopping the input of image data lines is off"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DINLC_A::_0)
    }
    #[doc = "Count mode for stopping the input of image data lines is on"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DINLC_A::_1)
    }
}
#[doc = "Field `DINRCMD` writer - Input Image Data Lines Resume Command This bit is valid only when the count mode for stopping the input of image data lines is on. Setting this bit to 1 resumes reading input image data. This bit is always read as 0."]
pub type DINRCMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, JIFECNT_SPEC, bool, O>;
#[doc = "Field `DINRINI` reader - Address Initialization when Resuming Input of Image Data Lines This bit is only valid when the count mode for stopping the input of image data lines is on. Set this bit before writing 1 to the data-line resume command bit."]
pub type DINRINI_R = crate::BitReader<DINRINI_A>;
#[doc = "Address Initialization when Resuming Input of Image Data Lines This bit is only valid when the count mode for stopping the input of image data lines is on. Set this bit before writing 1 to the data-line resume command bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DINRINI_A {
    #[doc = "0: The transfer address is not initialized when the input of image data lines is restarted"]
    _0 = 0,
    #[doc = "1: The transfer address is initialized when the input of image data lines is restarted"]
    _1 = 1,
}
impl From<DINRINI_A> for bool {
    #[inline(always)]
    fn from(variant: DINRINI_A) -> Self {
        variant as u8 != 0
    }
}
impl DINRINI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINRINI_A {
        match self.bits {
            false => DINRINI_A::_0,
            true => DINRINI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DINRINI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DINRINI_A::_1
    }
}
#[doc = "Field `DINRINI` writer - Address Initialization when Resuming Input of Image Data Lines This bit is only valid when the count mode for stopping the input of image data lines is on. Set this bit before writing 1 to the data-line resume command bit."]
pub type DINRINI_W<'a, const O: u8> = crate::BitWriter<'a, u32, JIFECNT_SPEC, DINRINI_A, O>;
impl<'a, const O: u8> DINRINI_W<'a, O> {
    #[doc = "The transfer address is not initialized when the input of image data lines is restarted"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DINRINI_A::_0)
    }
    #[doc = "The transfer address is initialized when the input of image data lines is restarted"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DINRINI_A::_1)
    }
}
#[doc = "Field `JOUTSWAP` reader - Byte/Halfword/Word Swap Output coded data in compression is swapped."]
pub type JOUTSWAP_R = crate::FieldReader<u8, JOUTSWAP_A>;
#[doc = "Byte/Halfword/Word Swap Output coded data in compression is swapped.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JOUTSWAP_A {
    #[doc = "0: (1) (2) (3) (4) (5) (6) (7) (8)"]
    _000 = 0,
    #[doc = "1: (2) (1) (4) (3) (6) (5) (8) (7) \\[Byte swap\\]"]
    _001 = 1,
    #[doc = "2: (3) (4) (1) (2) (7) (8) (5) (6) \\[Halfword swap\\]"]
    _010 = 2,
    #[doc = "3: (4) (3) (2) (1) (8) (7) (6) (5) \\[Halfword - byte swap\\]"]
    _011 = 3,
    #[doc = "4: (5) (6) (7) (8) (1) (2) (3) (4) \\[Word swap\\]"]
    _100 = 4,
    #[doc = "5: (6) (5) (8) (7) (2) (1) (4) (3) \\[Word - byte swap\\]"]
    _101 = 5,
    #[doc = "6: (7) (8) (5) (6) (3) (4) (1) (2) \\[Word - Halfword swap\\]"]
    _110 = 6,
    #[doc = "7: (8) (7) (6) (5) (4) (3) (2) (1) \\[Word - Word - byte swap\\]"]
    _111 = 7,
}
impl From<JOUTSWAP_A> for u8 {
    #[inline(always)]
    fn from(variant: JOUTSWAP_A) -> Self {
        variant as _
    }
}
impl JOUTSWAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JOUTSWAP_A {
        match self.bits {
            0 => JOUTSWAP_A::_000,
            1 => JOUTSWAP_A::_001,
            2 => JOUTSWAP_A::_010,
            3 => JOUTSWAP_A::_011,
            4 => JOUTSWAP_A::_100,
            5 => JOUTSWAP_A::_101,
            6 => JOUTSWAP_A::_110,
            7 => JOUTSWAP_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == JOUTSWAP_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == JOUTSWAP_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == JOUTSWAP_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == JOUTSWAP_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == JOUTSWAP_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == JOUTSWAP_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == JOUTSWAP_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == JOUTSWAP_A::_111
    }
}
#[doc = "Field `JOUTSWAP` writer - Byte/Halfword/Word Swap Output coded data in compression is swapped."]
pub type JOUTSWAP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, JIFECNT_SPEC, u8, JOUTSWAP_A, 3, O>;
impl<'a, const O: u8> JOUTSWAP_W<'a, O> {
    #[doc = "(1) (2) (3) (4) (5) (6) (7) (8)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(JOUTSWAP_A::_000)
    }
    #[doc = "(2) (1) (4) (3) (6) (5) (8) (7) \\[Byte swap\\]"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(JOUTSWAP_A::_001)
    }
    #[doc = "(3) (4) (1) (2) (7) (8) (5) (6) \\[Halfword swap\\]"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(JOUTSWAP_A::_010)
    }
    #[doc = "(4) (3) (2) (1) (8) (7) (6) (5) \\[Halfword - byte swap\\]"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(JOUTSWAP_A::_011)
    }
    #[doc = "(5) (6) (7) (8) (1) (2) (3) (4) \\[Word swap\\]"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(JOUTSWAP_A::_100)
    }
    #[doc = "(6) (5) (8) (7) (2) (1) (4) (3) \\[Word - byte swap\\]"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(JOUTSWAP_A::_101)
    }
    #[doc = "(7) (8) (5) (6) (3) (4) (1) (2) \\[Word - Halfword swap\\]"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(JOUTSWAP_A::_110)
    }
    #[doc = "(8) (7) (6) (5) (4) (3) (2) (1) \\[Word - Word - byte swap\\]"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(JOUTSWAP_A::_111)
    }
}
impl R {
    #[doc = "Bits 0:2 - Byte/Halfword Swap"]
    #[inline(always)]
    pub fn dinswap(&self) -> DINSWAP_R {
        DINSWAP_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Count Mode Setting for Stopping Input Image Data Lines"]
    #[inline(always)]
    pub fn dinlc(&self) -> DINLC_R {
        DINLC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Address Initialization when Resuming Input of Image Data Lines This bit is only valid when the count mode for stopping the input of image data lines is on. Set this bit before writing 1 to the data-line resume command bit."]
    #[inline(always)]
    pub fn dinrini(&self) -> DINRINI_R {
        DINRINI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Byte/Halfword/Word Swap Output coded data in compression is swapped."]
    #[inline(always)]
    pub fn joutswap(&self) -> JOUTSWAP_R {
        JOUTSWAP_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Byte/Halfword Swap"]
    #[inline(always)]
    #[must_use]
    pub fn dinswap(&mut self) -> DINSWAP_W<0> {
        DINSWAP_W::new(self)
    }
    #[doc = "Bit 4 - Count Mode Setting for Stopping Input Image Data Lines"]
    #[inline(always)]
    #[must_use]
    pub fn dinlc(&mut self) -> DINLC_W<4> {
        DINLC_W::new(self)
    }
    #[doc = "Bit 5 - Input Image Data Lines Resume Command This bit is valid only when the count mode for stopping the input of image data lines is on. Setting this bit to 1 resumes reading input image data. This bit is always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn dinrcmd(&mut self) -> DINRCMD_W<5> {
        DINRCMD_W::new(self)
    }
    #[doc = "Bit 6 - Address Initialization when Resuming Input of Image Data Lines This bit is only valid when the count mode for stopping the input of image data lines is on. Set this bit before writing 1 to the data-line resume command bit."]
    #[inline(always)]
    #[must_use]
    pub fn dinrini(&mut self) -> DINRINI_W<6> {
        DINRINI_W::new(self)
    }
    #[doc = "Bits 8:10 - Byte/Halfword/Word Swap Output coded data in compression is swapped."]
    #[inline(always)]
    #[must_use]
    pub fn joutswap(&mut self) -> JOUTSWAP_W<8> {
        JOUTSWAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "JPEG Interface Compression Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jifecnt](index.html) module"]
pub struct JIFECNT_SPEC;
impl crate::RegisterSpec for JIFECNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jifecnt::R](R) reader structure"]
impl crate::Readable for JIFECNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jifecnt::W](W) writer structure"]
impl crate::Writable for JIFECNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JIFECNT to value 0"]
impl crate::Resettable for JIFECNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
